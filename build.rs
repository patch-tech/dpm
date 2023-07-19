fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .compile(&["proto/dpm_agent.proto"], &["proto"])?;

    tonic_build::compile_protos("proto/dpm_agent.proto")?;

    built::write_built_file().expect("Failed to acquire build-time information");

    Ok(())
}
