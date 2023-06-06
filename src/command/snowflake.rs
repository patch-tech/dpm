use serde::Deserialize;

mod dpm_agent {
    tonic::include_proto!("dpm_agent");
}
use dpm_agent::{dpm_agent_client::DpmAgentClient, query, Query, SnowflakeConnectionParams};

use crate::command::snowflake::dpm_agent::query::SelectExpression;

pub struct SnowflakeDescription {}

/// AFAICT this is the best that can be done with `#[derive(Deserialize)]`. An
/// improvement on this would be to have `data_type` deserialized into an enum
/// `DataType` whose variants store type-specific parameters
/// (`character_maximum_length` for `Varchar`, etc.). That might require a
/// handwritten `Deserialize` impl, though.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(dead_code)] // TODO(PAT-3375): Remove
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
    /// TODO: Add #[serde(deserialize_with = "...")] attribute
    is_nullable: String,
    /// Data type of the column
    /// TODO: Add #[serde(deserialize_with = "...")] attribute, OR use enum with unit variants
    data_type: String,
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

pub async fn describe(
    _tables: Vec<String>,
    _schemas: Vec<String>,
    _output: Option<String>,
) -> SnowflakeDescription {
    let grpc_url = format!(
        "http://{}:{}",
        std::env::var("DPM_AGENT_HOST").unwrap_or("[::1]".into()),
        std::env::var("DPM_AGENT_PORT").unwrap_or("50051".into())
    );

    println!("connecting to dpm-agent at {} ...", grpc_url);
    let mut client = match DpmAgentClient::connect(grpc_url).await {
        Ok(client) => client,
        Err(e) => panic!("connection failed: {:?}", e),
    };
    println!("connected to dpm-agent");

    let connection_params =
        dpm_agent::connection_request::ConnectionParams::SnowflakeConnectionParams(
            SnowflakeConnectionParams {
                account: std::env::var("SNOWSQL_ACCOUNT").unwrap(),
                database: std::env::var("SNOWSQL_DATABASE").unwrap(),
                user: std::env::var("SNOWSQL_USER").unwrap(),
                password: std::env::var("SNOWSQL_PWD").unwrap(),
                schema: "INFORMATION_SCHEMA".into(), // std::env::var("SNOWSQL_SCHEMA").unwrap(),
            },
        );
    let req = tonic::Request::new(dpm_agent::ConnectionRequest {
        connection_params: Some(connection_params),
    });

    println!("creating connection");
    let connection_response = client.create_connection(req).await.unwrap().into_inner();
    println!("connection created");

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
        argument: Some(query::Expression {
            ex_type: Some(query::expression::ExType::Field(query::FieldReference {
                field_name: c,
            })),
        }),
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
        argument: Some(query::Expression {
            ex_type: Some(query::expression::ExType::Field(query::FieldReference {
                field_name: c,
            })),
        }),
        direction: None,
    })
    .collect();

    let introspection = Query {
        connection_id: connection_response.connection_id,
        select_from: "COLUMNS".into(),
        select,
        filter: None,
        group_by: vec![],
        order_by,
        limit: None,
    };

    println!("introspecting ...");
    let intr = client
        .execute_query(introspection)
        .await
        .unwrap()
        .into_inner();

    let data: Vec<InformationSchemaColumnsRow> =
        serde_json::from_str(intr.json_data.as_str()).unwrap();
    println!("{:?}", data);

    SnowflakeDescription {}
}
