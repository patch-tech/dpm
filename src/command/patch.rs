use directories::ProjectDirs;
use serde_json::{self, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use chrono::Utc;
use serde::Deserialize;

use crate::descriptor::{
    BooleanFieldType, Constraints, DataPackage, DataResource, DateFieldType, DateTimeFieldType,
    NumberFieldType, StringFieldFormat, StringFieldType, TableLocation, TableSchema,
    TableSchemaField,
};

/// Data types supported by the Patch Backend
#[derive(Debug, Deserialize)]
enum PatchType {
    String,
    Float,
    Int,
    Boolean,
    ID,
    BigInt,
    Byte,
    Date,
    DateTime,
    Decimal,
    Ksuid,
}

/// Patch credentials recieved from the CLI project directory
struct PatchCredentials {
    /// Patch bearer token used to query the Patch config api
    auth_token: String,
    /// Current active source in the patch cli
    source_id: String,
}

// Structure of data returned by the Patch config api
#[derive(Debug, Deserialize)]
struct PatchResponse {
    data: Data,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    dataset_by_name: Dataset,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Dataset {
    tables: Vec<Table>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Table {
    name: String,
    columns: Vec<Column>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Column {
    name: String,
    graphql_type: PatchType,
    nullable: bool,
}

fn get_patch_credentials() -> PatchCredentials {
    let patch_dir = ProjectDirs::from("", "", "patch-cli")
        .expect("Failed to find patch-cli project directory")
        .config_dir()
        .to_path_buf();

    let auth_contents = read_file_contents(patch_dir.join("auth.json"), "Failed to read auth file");
    let auth_json: Value = serde_json::from_str(&auth_contents).expect("Failed to parse JSON");
    let auth_token = auth_json
        .get("access_token")
        .expect("Failed to get access token")
        .as_str()
        .expect("Access token is not a string")
        .to_string();

    let source_contents =
        read_file_contents(patch_dir.join("source.json"), "Failed to read source file");
    let source_file: Value = serde_json::from_str(&source_contents).expect("Failed to parse JSON");
    let source_id = source_file
        .get("active_source_id")
        .expect("Failed to get source id")
        .as_str()
        .expect("Source id is not a string")
        .to_string();

    PatchCredentials {
        auth_token,
        source_id,
    }
}

fn read_file_contents(file_path: PathBuf, error_message: &str) -> String {
    let mut file = File::open(file_path).expect(error_message);
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    contents
}

/// Queries Patch for a dataset schema, returning it as a data package object.
pub async fn describe(package_name: String, dataset: String) -> DataPackage {
    let patch_credentials = get_patch_credentials();

    eprintln!("connecting to patch");
    let dataset = introspection_query(patch_credentials, &dataset)
        .await
        .data
        .dataset_by_name;

    let mut package = DataPackage::from(dataset);
    package.name = Some(package_name.parse().unwrap());
    package
}

/// Queries the Patch config api for a dataset by name, returning dataset schema information
async fn introspection_query(patch_credentials: PatchCredentials, dataset: &str) -> PatchResponse {
    let query = serde_json::json!({
        "query": format!("
        {{
            datasetByName(input: {{ sourceId: \"{source_id}\", datasetName: \"{dataset_name}\" }}) {{
                id
                tables {{
                    name
                    columns {{
                        name
                        graphqlType
                        nullable
                    }}
                    primaryKey {{
                        name
                    }}
                }}
            }}
        }}
    ", source_id = patch_credentials.source_id, dataset_name = dataset)
    });
    let client = reqwest::Client::new();
    let request = client
        .post("https://api.patch.tech/graphql")
        .header("Authorization", patch_credentials.auth_token)
        .header("Content-Type", "application/json");

    let response = request.body(query.to_string()).send().await;

    let body = response
        .expect("REASON")
        .text()
        .await
        .expect("could not get body");
    let response: PatchResponse = serde_json::from_str(&body).expect(
        "could not deserialize JSON (try rerunning a `pat` command to refresh your credentials)",
    );
    response
}

impl From<Dataset> for DataPackage {
    fn from(dataset: Dataset) -> Self {
        #[derive(Clone, Copy, Hash, PartialEq, Eq)]
        struct TableId<'a> {
            table: &'a str,
        }

        let mut fields_by_table: HashMap<TableId, Vec<TableSchemaField>> = HashMap::new();
        for table in &dataset.tables {
            for column in &table.columns {
                let fields = fields_by_table
                    .entry(TableId { table: &table.name })
                    .or_insert(Vec::new());

                let base_constraints = Constraints {
                    required: Some(column.nullable),
                    ..Default::default()
                };

                fields.push(match column.graphql_type {
                    PatchType::Byte => TableSchemaField::StringField {
                        constraints: Some(Constraints { ..base_constraints }),
                        description: None,
                        example: None,
                        format: StringFieldFormat::Binary,
                        name: column.name.clone(),
                        rdf_type: None,
                        title: None,
                        type_: Some(StringFieldType::String),
                    },
                    PatchType::Boolean => TableSchemaField::BooleanField {
                        constraints: Some(base_constraints),
                        description: None,
                        example: None,
                        false_values: Vec::new(),
                        format: Default::default(),
                        name: column.name.clone(),
                        rdf_type: None,
                        title: None,
                        true_values: Vec::new(),
                        type_: BooleanFieldType::Boolean,
                    },
                    PatchType::Date => TableSchemaField::DateField {
                        constraints: Some(base_constraints),
                        description: None,
                        example: None,
                        format: Default::default(),
                        name: column.name.clone(),
                        rdf_type: None,
                        title: None,
                        type_: DateFieldType::Date,
                    },
                    PatchType::Float => TableSchemaField::NumberField {
                        bare_number: true,
                        constraints: Some(base_constraints),
                        decimal_char: None,
                        description: None,
                        example: None,
                        format: Default::default(),
                        group_char: None,
                        name: column.name.clone(),
                        rdf_type: None,
                        title: None,
                        type_: NumberFieldType::Number,
                    },
                    PatchType::Int => TableSchemaField::NumberField {
                        bare_number: true,
                        constraints: Some(base_constraints),
                        decimal_char: None,
                        description: None,
                        example: None,
                        format: Default::default(),
                        group_char: None,
                        name: column.name.clone(),
                        rdf_type: None,
                        title: None,
                        type_: NumberFieldType::Number,
                    },
                    PatchType::BigInt => TableSchemaField::NumberField {
                        bare_number: true,
                        constraints: Some(base_constraints),
                        decimal_char: None,
                        description: None,
                        example: None,
                        format: Default::default(),
                        group_char: None,
                        name: column.name.clone(),
                        rdf_type: None,
                        title: None,
                        type_: NumberFieldType::Number,
                    },
                    PatchType::Decimal => TableSchemaField::NumberField {
                        bare_number: true,
                        constraints: Some(base_constraints),
                        decimal_char: None,
                        description: None,
                        example: None,
                        format: Default::default(),
                        group_char: None,
                        name: column.name.clone(),
                        rdf_type: None,
                        title: None,
                        type_: NumberFieldType::Number,
                    },
                    PatchType::String => TableSchemaField::StringField {
                        constraints: Some(Constraints { ..base_constraints }),
                        description: None,
                        example: None,
                        format: StringFieldFormat::Default,
                        name: column.name.clone(),
                        rdf_type: None,
                        title: None,
                        type_: Some(StringFieldType::String),
                    },
                    PatchType::DateTime => TableSchemaField::DateTimeField {
                        constraints: Some(base_constraints),
                        description: None,
                        example: None,
                        format: Default::default(),
                        name: column.name.clone(),
                        rdf_type: None,
                        title: None,
                        type_: DateTimeFieldType::Datetime,
                    },
                    PatchType::ID => TableSchemaField::StringField {
                        constraints: Some(Constraints { ..base_constraints }),
                        description: None,
                        example: None,
                        format: StringFieldFormat::Default,
                        name: column.name.clone(),
                        rdf_type: None,
                        title: None,
                        type_: Some(StringFieldType::String),
                    },
                    PatchType::Ksuid => TableSchemaField::StringField {
                        constraints: Some(Constraints { ..base_constraints }),
                        description: None,
                        example: None,
                        format: StringFieldFormat::Default,
                        name: column.name.clone(),
                        rdf_type: None,
                        title: None,
                        type_: Some(StringFieldType::String),
                    },
                })
            }
        }

        let mut tables: HashMap<TableId, DataResource> = HashMap::new();
        for (table_id, fields) in fields_by_table {
            let table_schema = TableSchema::Object {
                fields,
                missing_values: Vec::new(),
                primary_key: None,
            };

            tables.entry(table_id).or_insert(DataResource {
                bytes: None,
                data: None,
                description: None,
                encoding: None,
                format: None,
                hash: None,
                homepage: None,
                licenses: Vec::new(),
                mediatype: None,
                name: Some(table_id.table.into()),
                path: Some("https://api.patch.tech/query/graphql".into()),
                profile: "data-resource".into(),
                schema: Some(table_schema),
                location: TableLocation::Patch,
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
