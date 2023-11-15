pub enum AllowListItem {
    BigQueryTable(String),
    SnowflakeSchema(String),
    SnowflakeTable {
        schema: Option<String>,
        table: String,
    },
}
