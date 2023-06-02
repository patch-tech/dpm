pub mod dpm_agent {
    tonic::include_proto!("dpm_agent");
}
use dpm_agent::dpm_agent_client::DpmAgentClient;

pub struct SnowflakeDescription {
    // database: String,
    // tables: Vec<Table>,
}

pub async fn describe(
    _tables: Vec<String>,
    _schemas: Vec<String>,
    _output: Option<String>,
) -> SnowflakeDescription {
    // Read connection params from env vars.
    // Instantiate client.
    // Run introspection query, obtain SnowflakeDescription

    let grpc_host = std::env::var("DPM_AGENT_HOST").unwrap_or("[::1]".to_string());
    let grpc_port = std::env::var("DPM_AGENT_PORT").unwrap_or("50051".to_string());
    let grpc_url = format!("http://{}:{}", grpc_host, grpc_port);

    println!("connecting to {} ...", grpc_url);
    match DpmAgentClient::connect(grpc_url).await {
        Ok(_) => println!("connected!"),
        Err(e) => println!("connection failed: {:?}", e),
    }

    SnowflakeDescription {}
}
