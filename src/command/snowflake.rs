use std::{collections::HashMap, env};

use chrono::Utc;
use serde::Deserialize;

use crate::command::snowflake::dpm_agent::query::SelectExpression;
use crate::descriptor::{
    AnyFieldType, ArrayFieldType, BooleanFieldType, Constraints, DataPackage, DataResource,
    DateFieldType, DateTimeFieldType, NumberFieldType, ObjectFieldType, ResourceName,
    StringFieldFormat, StringFieldType, TableSchema, TableSchemaField, TimeFieldType,
};

mod dpm_agent {
    tonic::include_proto!("dpm_agent");
}
use dpm_agent::{dpm_agent_client::DpmAgentClient, query, Query, SnowflakeConnectionParams};

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
    _tables: Vec<String>,
    _schemas: Vec<String>,
    _output: Option<String>,
) -> DataPackage {
    let grpc_url = format!(
        "http://{}:{}",
        env::var("DPM_AGENT_HOST").unwrap_or("[::1]".into()),
        env::var("DPM_AGENT_PORT").unwrap_or("50051".into())
    );

    eprintln!("connecting to dpm-agent at {} ...", grpc_url);
    let mut client = match DpmAgentClient::connect(grpc_url).await {
        Ok(client) => client,
        Err(e) => panic!("connection failed: {:?}", e),
    };
    eprintln!("connected to dpm-agent");

    let connection_params =
        dpm_agent::connection_request::ConnectionParams::SnowflakeConnectionParams(
            SnowflakeConnectionParams {
                account: env::var("SNOWSQL_ACCOUNT").unwrap(),
                database: env::var("SNOWSQL_DATABASE").unwrap(),
                user: env::var("SNOWSQL_USER").unwrap(),
                password: env::var("SNOWSQL_PWD").unwrap(),
                schema: "INFORMATION_SCHEMA".into(),
            },
        );
    let req = tonic::Request::new(dpm_agent::ConnectionRequest {
        connection_params: Some(connection_params),
    });

    eprintln!("creating connection");
    let connection_response = client.create_connection(req).await.unwrap().into_inner();
    eprintln!("connection created");

    eprintln!("introspecting ...");
    let response = client
        .execute_query(introspection_query(&connection_response.connection_id))
        .await;
    let query_result = match response {
        Ok(response) => response.into_inner(),
        Err(e) => panic!("error during `ExecuteQuery`: {:?}", e),
    };

    let data: Vec<InformationSchemaColumnsRow> =
        match serde_json::from_str(query_result.json_data.as_str()) {
            Ok(v) => v,
            Err(e) => panic!("error deserializing `QueryResult.jsonData`: {:?}", e),
        };

    DataPackage::from(data)
}

/// Forms an introspection `Query` to run `ExecuteQuery` with using a connection
/// created previously.
///
/// Given that
/// [`INFORMATION_SCHEMA`](https://en.wikipedia.org/wiki/Information_schema) is
/// fairly standardized, this query and its results not be very Snowflake-specific.
fn introspection_query(connection_id: &str) -> Query {
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
        // TODO(PAT-3376): Add filter clauses based on CLI args
        filter: None,
        group_by: Vec::new(),
        order_by,
        limit: None,
    }
}

impl From<Vec<InformationSchemaColumnsRow>> for DataPackage {
    fn from(rows: Vec<InformationSchemaColumnsRow>) -> Self {
        #[derive(Clone, Copy, Hash, PartialEq, Eq)]
        struct TableId<'a> {
            schema: &'a str,
            table: &'a str,
        }

        let mut fields_by_table: HashMap<TableId, Vec<TableSchemaField>> = HashMap::new();
        for row in &rows {
            let fields = fields_by_table
                .entry(TableId {
                    schema: &row.table_schema,
                    table: &row.table_name,
                })
                .or_insert(Vec::new());

            // Ignore GEOGRAPHY and GEOMETRY columns. They're currently unsupported.
            match row.data_type {
                SnowflakeType::Geography | SnowflakeType::Geometry => continue,
                _ => (),
            }

            let required = Some(row.is_nullable == "NO");

            fields.push(match row.data_type {
                SnowflakeType::Binary => TableSchemaField::StringField {
                    constraints: Some(Constraints {
                        enum_: None,
                        max_length: Some(MAX_BINARY_STRING_SIZE),
                        min_length: None,
                        pattern: None,
                        required,
                        unique: None,
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
                    constraints: Some(Constraints {
                        enum_: None,
                        max_length: None,
                        min_length: None,
                        pattern: None,
                        required,
                        unique: None,
                    }),
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
                    constraints: Some(Constraints {
                        enum_: None,
                        max_length: None,
                        min_length: None,
                        pattern: None,
                        required,
                        unique: None,
                    }),
                    description: row.comment.clone(),
                    example: None,
                    format: Default::default(),
                    name: row.column_name.clone(),
                    rdf_type: None,
                    title: None,
                    // TODO: file typify issue about these single-variant enums leading to annoying codegen like this
                    type_: DateFieldType::Date,
                },
                SnowflakeType::Float => TableSchemaField::NumberField {
                    bare_number: true,
                    constraints: Some(Constraints {
                        enum_: None,
                        max_length: None,
                        min_length: None,
                        pattern: None,
                        required,
                        unique: None,
                    }),
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
                    constraints: Some(Constraints {
                        enum_: None,
                        max_length: None,
                        min_length: None,
                        pattern: None,
                        required,
                        unique: None,
                    }),
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
                        enum_: None,
                        max_length: Some(2_i64.pow(24)),
                        min_length: None,
                        pattern: None,
                        required,
                        unique: None,
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
                    constraints: Some(Constraints {
                        enum_: None,
                        max_length: None,
                        min_length: None,
                        pattern: None,
                        required,
                        unique: None,
                    }),
                    description: row.comment.clone(),
                    example: None,
                    format: Default::default(),
                    name: row.column_name.clone(),
                    rdf_type: None,
                    title: None,
                    type_: TimeFieldType::Time,
                },
                SnowflakeType::TimestampLtz
                | SnowflakeType::TimestampNtz
                | SnowflakeType::TimestampTz => TableSchemaField::DateTimeField {
                    constraints: Some(Constraints {
                        enum_: None,
                        max_length: None,
                        min_length: None,
                        pattern: None,
                        required,
                        unique: None,
                    }),
                    description: row.comment.clone(),
                    example: None,
                    format: Default::default(),
                    name: row.column_name.clone(),
                    rdf_type: None,
                    title: None,
                    type_: DateTimeFieldType::Datetime,
                },
                SnowflakeType::Array => TableSchemaField::ArrayField {
                    constraints: Some(Constraints {
                        enum_: None,
                        max_length: None,
                        min_length: None,
                        pattern: None,
                        required,
                        unique: None,
                    }),
                    description: row.comment.clone(),
                    example: None,
                    format: Default::default(),
                    name: row.column_name.clone(),
                    rdf_type: None,
                    title: None,
                    type_: ArrayFieldType::Array,
                },

                SnowflakeType::Object => TableSchemaField::ObjectField {
                    constraints: Some(Constraints {
                        enum_: None,
                        max_length: None,
                        min_length: None,
                        pattern: None,
                        required,
                        unique: None,
                    }),
                    description: row.comment.clone(),
                    example: None,
                    format: Default::default(),
                    name: row.column_name.clone(),
                    rdf_type: None,
                    title: None,
                    type_: ObjectFieldType::Object,
                },
                SnowflakeType::Variant => TableSchemaField::AnyField {
                    constraints: Some(Constraints {
                        enum_: None,
                        max_length: None,
                        min_length: None,
                        pattern: None,
                        required,
                        unique: None,
                    }),
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

        let mut tables: HashMap<TableId, DataResource> = HashMap::new();
        for (table_id, fields) in fields_by_table {
            let table_schema = TableSchema::Object {
                fields,
                missing_values: Vec::new(),
                primary_key: None,
            };

            let name: Option<ResourceName> = match table_id.table.parse() {
                Ok(name) => Some(name),
                Err(_e) => {
                    eprintln!(
                        "omitting table with invalid name: {}",
                        serde_json::to_string(table_id.table).unwrap()
                    );
                    continue;
                }
            };

            tables.entry(table_id).or_insert(DataResource {
                bytes: None,
                data: None,
                // TODO(PAT-3448): Get this from INFORMATION_SCHEMA.TABLES's COMMENT column.
                description: None,
                encoding: None,
                format: None,
                hash: None,
                homepage: None,
                licenses: Vec::new(),
                mediatype: None,
                name: name.clone(),
                path: Some(name.unwrap().parse().unwrap()),
                profile: "data-resource".into(),
                schema: Some(table_schema),
                sources: Vec::new(),
                title: None,
            });
        }

        DataPackage {
            contributors: Vec::new(),
            created: Some(Utc::now()),
            description: None,
            homepage: None,
            id: None,
            image: None,
            keywords: Vec::new(),
            licenses: Vec::new(),
            name: None,
            profile: "data-package".into(),
            resources: tables.into_values().collect(),
            sources: Vec::new(),
            title: None,
            version: "0.1.0".parse().unwrap(),
        }
    }
}
