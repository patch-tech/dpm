use serde_json::Value;
use std::env;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

use serde::Deserialize;

/// SNOWSQL variables present either in the environment or sops file.
#[derive(Deserialize)]
struct SnowflakeTestConfig {
    #[serde(rename = "account")]
    org_account: String,
    database: String,
    user: String,
    #[serde(rename = "pwd")]
    password: String,
}

pub fn exec_cmd(path: &Path, program: &str, args: &[&str]) -> String {
    let mut cmd_binding = Command::new(program);
    let cmd = cmd_binding.current_dir(path).args(args);

    let output = cmd
        .output()
        .expect(&format!("Failed to execute program \"{}\"", program));

    if !output.status.success() {
        let args_str = args
            .iter()
            .map(|a| a.replace("\"", "\\\""))
            .map(|a| format!("\"{}\"", a))
            .collect::<Vec<String>>()
            .join(" ");

        fn format_stream(s: &Vec<u8>) -> &str {
            if s.is_empty() {
                "<empty>\n"
            } else {
                std::str::from_utf8(s).unwrap()
            }
        }
        let message = format!(
            "Command (\"{}\" {}) failed ({})\n\n=== stdout === \n\n{}\n=== stderr ===\n\n{}\n==============\n",
            program,
            args_str,
            output.status,
            format_stream(&output.stdout),
            format_stream(&output.stderr)
        );

        panic!("{}", message);
    }

    String::from_utf8(output.stdout).unwrap()
}

/// Returns data read out of the sops-encrypted secrets file.
fn data_from_sops<T: serde::de::DeserializeOwned>(prefix: Option<&str>) -> T {
    let secret_file_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("secrets")
        .join("dpm.enc.env");

    let json = exec_cmd(
        &std::env::current_dir().unwrap(),
        "sops",
        // NB: This order is important! https://github.com/getsops/sops/issues/1259
        &[
            "--output-type",
            "json",
            "-d",
            &secret_file_path.to_string_lossy(),
        ],
    );
    let value = serde_json::from_str::<serde_json::Value>(&json)
        .expect("output of sops decryption was not JSON");
    let obj = value
        .as_object()
        .expect("output of sops decryption was not a JSON object");
    let entries = obj
        .iter()
        .map(|(key, value)| (key.to_owned(), value.as_str().unwrap().to_owned()));

    let result = if let Some(prefix) = prefix {
        envy::prefixed(prefix).from_iter(entries)
    } else {
        envy::from_iter(entries)
    };

    result.expect("sops file did not contain all expected variables")
}

/// Runs `dpm source create snowflake`, creating a new test source and returning
/// its name. Source details are expected in either SNOWSQL environment
/// variables or, failing that, those in the sops file are used. Targets
/// whatever API URL would be used by a regular `dpm` invocation.
///
/// NB: Until PAT-3694 is done, `dpm` does not indicate failure via its exit
/// status. As a result, this function may return normally (i.e., not panic)
/// even if source creation fails.
pub fn create_snowflake_source(current_dir: &Path) -> String {
    let source_name = format!(
        "integration-test_{}",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );

    let config = envy::prefixed("SNOWSQL_")
        .from_env::<SnowflakeTestConfig>()
        .unwrap_or_else(|e| {
            eprintln!(
                "Error obtaining Snowflake test source config from environment variables: {}",
                e
            );
            eprintln!("Falling back to getting Snowflake test source credentials via sops...");
            data_from_sops::<SnowflakeTestConfig>(Some("SNOWSQL_"))
        });

    let i = config
        .org_account
        .find("-")
        .expect("SNOWSQL_ACCOUNT should have shape {organization}-{account}");
    let (organization, account) = (config.org_account).split_at(i);
    let account = &account[1..];

    exec_cmd(
        current_dir,
        env!("CARGO_BIN_EXE_dpm"),
        &[
            "source",
            "create",
            "snowflake",
            "-n",
            &source_name,
            "--organization",
            organization,
            "--account",
            account,
            "--database",
            &config.database,
            "--user",
            &config.user,
            "--password",
            &config.password,
        ],
    );

    source_name
}

pub fn init_snowflake(current_dir: &PathBuf, source_name: &str) {
    let generated_dir = current_dir.join(Path::new("./tests/resources/generated"));

    exec_cmd(
        &generated_dir,
        env!("CARGO_BIN_EXE_dpm"),
        &[
            "init",
            "-o",
            "datapackage_snowflake.json",
            "--name",
            "test-snowflake",
            "--source",
            source_name,
            "snowflake",
            "--schema",
            "PUBLIC",
        ],
    );

    // assert generated directory is not empty
    assert!(
        std::path::Path::new("./tests/resources/generated/datapackage_snowflake.json").exists()
    );
    let descriptor_contents = read_to_string(Path::new(
        "./tests/resources/generated/datapackage_snowflake.json",
    ))
    .expect("Failed to read descriptor contents");
    let descriptor_value: Value =
        serde_json::from_str(&descriptor_contents).expect("Unable to parse JSON");

    // assert values in descriptor are correct (name, version)
    match &descriptor_value {
        Value::Object(map) => {
            let name = map.get("name").expect("Key 'name' does not exist");
            let version = map.get("version").expect("Key 'version' does not exist");
            assert_eq!(name, "test-snowflake");
            assert_eq!(version, "0.1.0");
        }
        _ => panic!("malformed dataset descriptor"),
    }
}

pub fn publish_snowflake_package(current_dir: &Path) {
    let generated_dir = current_dir.join(Path::new("./tests/resources/generated"));

    exec_cmd(
        &generated_dir,
        env!("CARGO_BIN_EXE_dpm"),
        &["publish", "-d", "datapackage_snowflake.json"],
    );
}

pub trait TargetTester {
    /// Builds data packages for all sources in target language and checks for their existance
    fn build_packages(&self, dir: &PathBuf, package_ref: &str);

    /// Installs package in a test file for given target
    fn install_packages(&self, dir: &PathBuf);

    /// Runs tests in given target's test project, validating compiled and executed queries
    fn test_packages(&self, dir: &PathBuf);

    /// Removes target specific generated directories
    fn cleanup(&self) -> std::io::Result<()>;
}
