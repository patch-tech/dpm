use anyhow::bail;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env::var as env_var;
use tonic::transport::{Channel, ClientTlsConfig};
use url::Url;

use crate::descriptor::{
    AnyFieldType, ArrayFieldType, BooleanFieldType, Constraints, DataPackage, DataResource,
    DateFieldType, DateTimeFieldType, Name, NumberFieldType, ObjectFieldType, SourcePath,
    StringFieldFormat, StringFieldType, TableSchema, TableSchemaField, TableSource, TimeFieldType,
};
use crate::{built_info, command::snowflake::dpm_agent::query::SelectExpression, env};

mod dpm_agent {
    tonic::include_proto!("dpm_agent");
}
use dpm_agent::{
    client_version::Client,
    dpm_agent_client::DpmAgentClient,
    query::{self, boolean_expression::BooleanOperator},
    ClientVersion, Query, SnowflakeConnectionParams,
};

/// Values in Snowflake `BINARY` columns may be at most 8 MiB.
/// 8 MiB, base64-encoded, is `4 * ceil(2.pow(23) / 3)` bytes,
/// per https://stackoverflow.com/questions/13378815/base64-length-calculation.
const MAX_BINARY_STRING_SIZE: i64 = 11_184_812;

/// Data types supported by the Snowflake DBMS.
/// Ref: https://docs.snowflake.com/en/sql-reference/data-types
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum SnowflakeType {
    Array,
    Binary,
    Boolean,
    Date,
    Float,
    Geography,
    Geometry,
    Number,
    Object,
    Text,
    Time,
    TimestampLtz,
    TimestampNtz,
    TimestampTz,
    Variant,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrganizationName(String);
impl std::str::FromStr for OrganizationName {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Source: https://docs.snowflake.com/en/user-guide/admin-account-identifier#organization-name
        // We assume that when ^ refers to "letters" that means [a-zA-Z].
        if regress::Regex::new("^[a-zA-Z][a-zA-Z0-9]*)$")
            .unwrap()
            .find(s)
            .is_none()
        {
            bail!("doesn't match pattern \"^[a-zA-Z][a-zA-Z0-9]*)$\"");
        }
        Ok(Self(s.to_string()))
    }
}

/// A row from a query to Snowflake's `INFORMATION_SCHEMA.COLUMNS` view. Ref:
/// https://docs.snowflake.com/en/sql-reference/info-schema/columns
///
/// AFAICT this is the best that can be done with `#[derive(Deserialize)]`. An
/// improvement on this would be to have `data_type` deserialized into an enum
/// `DataType`, with type parameters stored in struct variants
/// (`character_maximum_length` for `Varchar`, etc.). That might require a
/// handwritten `Deserialize` impl, though.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(dead_code)] // TODO(PAT-3446): Make full use of this data (for path, constraints, etc.)
struct InformationSchemaColumnsRow {
    /// Database that the table belongs to
    table_catalog: String,
    /// Schema that the table belongs to
    table_schema: String,
    /// Table or View that the column belongs to
    table_name: String,
    /// Name of the column
    column_name: String,
    /// Ordinal position of the column in the table
    ordinal_position: f32,
    /// "YES" if the column may contain NULL, "NO" otherwise
    is_nullable: String,
    /// Data type of the column
    data_type: SnowflakeType,
    /// Maximum length in characters of string columns.
    /// In Snowflake's VARCHAR(n Unicode code points), n <= 16_777_216 == 2.pow(24).
    character_maximum_length: Option<f32>,
    /// Numeric precision of numeric columns
    numeric_precision: Option<f32>,
    /// Radix of precision of numeric columns
    numeric_precision_radix: Option<f32>,
    /// Scale of numeric columns
    numeric_scale: Option<f32>,
    /// Comment for this column
    comment: Option<String>,
}

fn field_ref_expression(field_name: &str) -> query::Expression {
    query::Expression {
        ex_type: Some(query::expression::ExType::Field(query::FieldReference {
            field_name: field_name.into(),
        })),
    }
}

/// Introspects the named objects in Snowflake.
///
/// Table names must not be schema-qualified. Results are unioned together and
/// placed into a DataPackage.
pub async fn describe(
    package_name: Name,
    tables: Vec<String>,
    schemas: Vec<String>,
) -> DataPackage {
    let agent_url = env_var("DPM_AGENT_URL").unwrap_or("https://agent.dpm.sh".into());
    let agent_url = Url::parse(&agent_url)
        .unwrap_or_else(|_| panic!("DPM_AGENT_URL value not a valid URL: {}", agent_url));

    let mut endpoint = Channel::from_shared(agent_url.as_str().to_string()).unwrap();
    if agent_url.scheme() == "https" {
        let tls = ClientTlsConfig::new().domain_name(agent_url.host_str().unwrap());
        endpoint = endpoint.tls_config(tls).unwrap();
    }

    endpoint = endpoint.user_agent(env::user_agent()).unwrap();

    let channel = match endpoint.connect().await {
        Ok(channel) => {
            eprintln!("connected to {}", agent_url);
            channel
        }
        Err(e) => panic!("connection failed: {:?}", e),
    };

    let mut client = DpmAgentClient::new(channel);
    let client_version = ClientVersion {
        client: Client::Dpm.into(),
        code_version: built_info::PKG_VERSION.to_string(),
        dataset_version: "".into(),
    };

    let connection_params =
        dpm_agent::connection_request::ConnectionParams::SnowflakeConnectionParams(
            SnowflakeConnectionParams {
                account: env_var("SNOWSQL_ACCOUNT").unwrap(),
                database: env_var("SNOWSQL_DATABASE").unwrap(),
                user: env_var("SNOWSQL_USER").unwrap(),
                password: env_var("SNOWSQL_PWD").unwrap(),
                schema: "INFORMATION_SCHEMA".into(),
            },
        );
    let req = tonic::Request::new(dpm_agent::ConnectionRequest {
        connection_params: Some(connection_params),
        client_version: Some(client_version.clone()),
    });

    eprintln!("creating snowflake connection");
    let connection_response = client.create_connection(req).await.unwrap().into_inner();
    eprintln!("connection created");

    let connection_id = connection_response.connection_id.as_str();

    eprintln!("introspecting ...");
    let response = client
        .execute_query(introspection_query(
            connection_id,
            tables,
            schemas,
            &client_version,
        ))
        .await;
    let query_result = match response {
        Ok(response) => response.into_inner(),
        Err(e) => panic!("error during `ExecuteQuery`: {:?}", e),
    };

    eprintln!("disconnecting connection");
    let response = client
        .disconnect_connection(dpm_agent::DisconnectRequest {
            connection_id: connection_id.to_string(),
            client_version: Some(client_version.clone()),
        })
        .await;
    if response.is_err() {
        eprintln!(
            "Failed to disconnect connection {connection_id}, with error {:?}",
            response.err()
        );
    }
    eprintln!("connection disconnected");

    let data: Vec<InformationSchemaColumnsRow> =
        match serde_json::from_str(query_result.json_data.as_str()) {
            Ok(v) => v,
            Err(e) => panic!("error deserializing `QueryResult.jsonData`: {:?}", e),
        };

    let mut package = DataPackage::from(data);
    package.name = package_name;
    package
}

/// Forms an introspection `Query` to run `ExecuteQuery` with using a connection
/// created previously.
///
/// Given that
/// [`INFORMATION_SCHEMA`](https://en.wikipedia.org/wiki/Information_schema) is
/// fairly standardized, this query and its results not be very Snowflake-specific.
fn introspection_query(
    connection_id: &str,
    tables: Vec<String>,
    schemas: Vec<String>,
    client_version: &ClientVersion,
) -> Query {
    let select: Vec<SelectExpression> = [
        "table_catalog",
        "table_schema",
        "table_name",
        "column_name",
        "ordinal_position",
        "is_nullable",
        "data_type",
        "character_maximum_length",
        "numeric_precision",
        "numeric_precision_radix",
        "numeric_scale",
        "comment",
    ]
    .iter()
    .map(|c| c.to_uppercase())
    .map(|c| query::SelectExpression {
        argument: Some(field_ref_expression(&c)),
        alias: None,
    })
    .collect();

    let table_predicates = tables.iter().map(|table_name| {
        let be = query::BooleanExpression {
            op: BooleanOperator::Eq.into(),
            arguments: vec![
                field_ref_expression("TABLE_NAME"),
                query::Expression {
                    ex_type: Some(query::expression::ExType::Literal(query::Literal {
                        literal_type: Some(query::literal::LiteralType::String(table_name.into())),
                    })),
                },
            ],
        };

        query::Expression {
            ex_type: Some(query::expression::ExType::Condition(be)),
        }
    });

    let schema_predicates = schemas.iter().map(|schema_name| {
        let be = query::BooleanExpression {
            op: BooleanOperator::Eq.into(),
            arguments: vec![
                field_ref_expression("TABLE_SCHEMA"),
                query::Expression {
                    ex_type: Some(query::expression::ExType::Literal(query::Literal {
                        literal_type: Some(query::literal::LiteralType::String(schema_name.into())),
                    })),
                },
            ],
        };

        query::Expression {
            ex_type: Some(query::expression::ExType::Condition(be)),
        }
    });

    let filter = if tables.is_empty() && schemas.is_empty() {
        None
    } else {
        Some(query::BooleanExpression {
            op: BooleanOperator::Or.into(),
            arguments: table_predicates.chain(schema_predicates).collect(),
        })
    };

    let order_by: Vec<query::OrderByExpression> = [
        "table_catalog",
        "table_schema",
        "table_name",
        "ordinal_position",
    ]
    .iter()
    .map(|c| c.to_uppercase())
    .map(|c| query::OrderByExpression {
        argument: Some(field_ref_expression(&c)),
        direction: None,
    })
    .collect();

    Query {
        connection_id: connection_id.into(),
        select_from: "COLUMNS".into(),
        select,
        filter,
        group_by: Vec::new(),
        order_by,
        limit: None,
        dry_run: Some(false),
        client_version: Some(client_version.clone()),
    }
}

impl DataPackage {
    fn from(rows: Vec<InformationSchemaColumnsRow>) -> Self {
        #[derive(Clone, Copy, Hash, PartialEq, Ord, PartialOrd, Eq)]
        struct TableId<'a> {
            database: &'a str,
            schema: &'a str,
            table: &'a str,
        }

        let mut fields_by_table: BTreeMap<TableId, Vec<TableSchemaField>> = BTreeMap::new();
        for row in &rows {
            let fields = fields_by_table
                .entry(TableId {
                    database: &row.table_catalog,
                    schema: &row.table_schema,
                    table: &row.table_name,
                })
                .or_insert(Vec::new());

            // Ignore GEOGRAPHY and GEOMETRY columns. They're currently unsupported.
            match row.data_type {
                SnowflakeType::Geography | SnowflakeType::Geometry => {
                    eprintln!(
                        "warning: omitting column \"{}\" of unsupported type {:?} from table \"{}\".\"{}\".\"{}\"",
                        row.column_name,
                        row.data_type,
                        row.table_catalog,
                        row.table_schema,
                        row.table_name,
                        );
                    continue;
                }
                _ => (),
            }

            let base_constraints = Constraints {
                required: Some(row.is_nullable == "NO"),
                ..Default::default()
            };

            fields.push(match row.data_type {
                SnowflakeType::Binary => TableSchemaField::StringField {
                    constraints: Some(Constraints {
                        max_length: Some(MAX_BINARY_STRING_SIZE),
                        ..base_constraints
                    }),
                    description: row.comment.clone(),
                    example: None,
                    format: StringFieldFormat::Binary,
                    name: row.column_name.clone(),
                    rdf_type: None,
                    title: None,
                    type_: Some(StringFieldType::String),
                },
                SnowflakeType::Boolean => TableSchemaField::BooleanField {
                    constraints: Some(base_constraints),
                    description: row.comment.clone(),
                    example: None,
                    false_values: Vec::new(),
                    format: Default::default(),
                    name: row.column_name.clone(),
                    rdf_type: None,
                    title: None,
                    true_values: Vec::new(),
                    type_: BooleanFieldType::Boolean,
                },
                SnowflakeType::Date => TableSchemaField::DateField {
                    constraints: Some(base_constraints),
                    description: row.comment.clone(),
                    example: None,
                    name: row.column_name.clone(),
                    rdf_type: None,
                    title: None,
                    type_: DateFieldType::Date,
                },
                SnowflakeType::Float => TableSchemaField::NumberField {
                    bare_number: true,
                    constraints: Some(base_constraints),
                    decimal_char: None,
                    description: row.comment.clone(),
                    example: None,
                    format: Default::default(),
                    group_char: None,
                    name: row.column_name.clone(),
                    rdf_type: None,
                    title: None,
                    type_: NumberFieldType::Number,
                },
                SnowflakeType::Number => TableSchemaField::NumberField {
                    bare_number: true,
                    constraints: Some(base_constraints),
                    decimal_char: None,
                    description: row.comment.clone(),
                    example: None,
                    format: Default::default(),
                    group_char: None,
                    name: row.column_name.clone(),
                    rdf_type: None,
                    title: None,
                    type_: NumberFieldType::Number,
                },
                SnowflakeType::Text => TableSchemaField::StringField {
                    constraints: Some(Constraints {
                        max_length: Some(2_i64.pow(24)),
                        ..base_constraints
                    }),
                    description: row.comment.clone(),
                    example: None,
                    format: StringFieldFormat::Default,
                    name: row.column_name.clone(),
                    rdf_type: None,
                    title: None,
                    type_: Some(StringFieldType::String),
                },
                SnowflakeType::Time => TableSchemaField::TimeField {
                    constraints: Some(base_constraints),
                    description: row.comment.clone(),
                    example: None,
                    name: row.column_name.clone(),
                    rdf_type: None,
                    title: None,
                    type_: TimeFieldType::Time,
                },
                SnowflakeType::TimestampLtz
                | SnowflakeType::TimestampNtz
                | SnowflakeType::TimestampTz => TableSchemaField::DateTimeField {
                    constraints: Some(base_constraints),
                    description: row.comment.clone(),
                    example: None,
                    name: row.column_name.clone(),
                    rdf_type: None,
                    title: None,
                    type_: DateTimeFieldType::Datetime,
                },
                SnowflakeType::Array => TableSchemaField::ArrayField {
                    constraints: Some(base_constraints),
                    description: row.comment.clone(),
                    example: None,
                    format: Default::default(),
                    name: row.column_name.clone(),
                    rdf_type: None,
                    title: None,
                    type_: ArrayFieldType::Array,
                },
                SnowflakeType::Object => TableSchemaField::ObjectField {
                    constraints: Some(base_constraints),
                    description: row.comment.clone(),
                    example: None,
                    format: Default::default(),
                    name: row.column_name.clone(),
                    rdf_type: None,
                    title: None,
                    type_: ObjectFieldType::Object,
                },
                SnowflakeType::Variant => TableSchemaField::AnyField {
                    constraints: Some(base_constraints),
                    description: row.comment.clone(),
                    example: None,
                    name: row.column_name.clone(),
                    rdf_type: None,
                    title: None,
                    type_: AnyFieldType::Any,
                },
                SnowflakeType::Geography | SnowflakeType::Geometry => {
                    unreachable!("unexpected: {:?}", row.data_type)
                }
            })
        }

        let mut tables: BTreeMap<TableId, DataResource> = BTreeMap::new();
        for (table_id, fields) in fields_by_table {
            let table_schema = TableSchema::Object {
                fields,
                missing_values: Vec::new(),
                primary_key: None,
            };

            tables.entry(table_id).or_insert(DataResource {
                // TODO(PAT-3448): Get this from INFORMATION_SCHEMA.TABLES's COMMENT column.
                description: None,
                name: table_id.table.into(),
                source: TableSource::new(SourcePath::Snowflake {
                    // 'schema' here is the organizational concept in relational
                    // databases...
                    schema: table_id.schema.into(),
                    table: table_id.table.into(),
                }),
                // ...whereas here 'schema' is the shape of the table.
                schema: Some(table_schema),
            });
        }

        DataPackage {
            id: uuid7::uuid7(),
            // replaced in `snowflake::describe`
            name: "placeholder".parse().unwrap(),
            dataset: tables.into_values().collect(),
            version: "0.1.0".parse().unwrap(),
            description: None,
        }
    }
}
