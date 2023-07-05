use std::env;
use std::fs::{self};
use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};

fn exec_cmd(path: &Path, cmd: &str, args: &[&str]) -> String {
    let cmd = Command::new(cmd)
        .current_dir(path)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");

    let mut stdout = cmd.stdout.expect("Failed to capture command output");
    let mut output = String::new();
    stdout
        .read_to_string(&mut output)
        .expect("Failed to read command output");

    output
}

fn cleanup() -> std::io::Result<()> {
    // Remove all generated files
    let path = "./tests/resources/generated/";
    fs::remove_dir_all(path)?;
    fs::create_dir_all(path)?;
    Ok(())
}

#[test]
fn build_patch() {
    if let Ok(current_dir) = env::current_dir() {
        let home_dir = current_dir.as_path();

        let _python_stdout = exec_cmd(
            &home_dir,
            "cargo",
            &[
                "run",
                "build-package",
                "-d",
                "./tests/resources/patch_datapackage.json",
                "-o",
                "./tests/resources/generated",
                "-y",
                "python",
            ],
        );
        let _nodejs_stdout = exec_cmd(
            &home_dir,
            "cargo",
            &[
                "run",
                "build-package",
                "-d",
                "./tests/resources/patch_datapackage.json",
                "-o",
                "./tests/resources/generated",
                "-y",
                "nodejs",
            ],
        );
        // assert generated directories are not empty
        assert!(
            !fs::read_dir("./tests/resources/generated/python/test-patch@0.1.0.0.1.0")
                .map_err(|e| format!("Failed to read directory: {}", e))
                .unwrap()
                .next()
                .is_none()
        );
        assert!(
            !fs::read_dir("./tests/resources/generated/nodejs/test-patch@0.1.0-0.1.0")
                .map_err(|e| format!("Failed to read directory: {}", e))
                .unwrap()
                .next()
                .is_none()
        );
        cleanup().expect("Failed to cleanup generated files");
    } else {
        eprintln!("Failed to get current directory");
    }
}
