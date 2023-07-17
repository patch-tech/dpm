use serde_json::Value;
use std::env;
use std::fs::read_to_string;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn exec_cmd(path: &Path, cmd: &str, args: &[&str]) -> String {
    let mut cmd_binding = Command::new(cmd);
    let cmd = cmd_binding.current_dir(path).args(args);

    let cmd_output = cmd
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let mut stdout = cmd_output.stdout.expect("Failed to capture command output");
    let mut output = String::new();
    stdout
        .read_to_string(&mut output)
        .expect("Failed to read command output");

    assert!(
        cmd.output()
            .expect("Failed to execute command")
            .status
            .success(),
        "Command failed with output:\n{}",
        output
    );
    output
}

pub fn describe_snowflake(current_dir: &PathBuf) {
    let generated_dir = current_dir.join(Path::new("./tests/resources/generated"));
    if env::var("SNOWSQL_ACCOUNT").is_ok()
        && env::var("SNOWSQL_USER").is_ok()
        && env::var("SNOWSQL_PWD").is_ok()
        && env::var("SNOWSQL_DATABASE").is_ok()
        && env::var("SNOWSQL_SCHEMA").is_ok()
    {
        exec_cmd(
            &generated_dir,
            "cargo",
            &[
                "run",
                "describe",
                "snowflake",
                "--name",
                "test-snowflake",
                "--schema",
                "PUBLIC",
            ],
        );
    } else {
        exec_cmd(
            &generated_dir,
            "bash",
            &[
                "-e",
                "-c",
                "sops exec-env ../../../secrets/dpm.enc.env 'cargo run describe snowflake --name test-snowflake --schema PUBLIC'",
            ],
        );
    }

    // assert generated directory is not empty
    assert!(std::path::Path::new("./tests/resources/generated/datapackage.json").exists());
    let datapackage_contents =
        read_to_string(Path::new("./tests/resources/generated/datapackage.json"))
            .expect("Failed to read descriptor contents");
    let data_package: Value =
        serde_json::from_str(&datapackage_contents).expect("Unable to parse JSON");

    // assert values in datapackage are correct (name, version, profile of first table)
    match &data_package {
        Value::Object(map) => {
            let name = map.get("name").expect("Key 'name' does not exist");
            let version = map.get("version").expect("Key 'version' does not exist");
            let first_table_name = map
                .get("resources")
                .expect("Key 'resources' does not exist")[0]
                .get("profile")
                .expect("Key 'profile' does not exist in first resource");
            assert_eq!(name, "test-snowflake");
            assert_eq!(version, "0.1.0");
            assert_eq!(first_table_name, "data-resource")
        }
        _ => panic!("Not an object"),
    }
}
pub trait TargetTester {
    /// Builds a data package in target language and checks for its existance
    fn build_patch(&self, dir: &PathBuf);

    /// Installs package in a test file for given target
    fn install_package(&self, dir: &PathBuf);

    /// Runs tests in given target's test project, validating compiled and executed queries
    fn test_package(&self, dir: &PathBuf);

    /// Removes target specific generated directories
    fn cleanup(&self) -> std::io::Result<()>;
}
