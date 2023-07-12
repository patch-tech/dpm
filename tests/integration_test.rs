use std::env;
use std::fs::{self};
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn exec_cmd(path: &Path, cmd: &str, args: &[&str]) -> String {
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

fn startup() -> std::io::Result<()> {
    let path = PathBuf::from("./tests/resources/generated/");
    fs::create_dir_all(&path)?;
    Ok(())
}

fn cleanup() -> std::io::Result<()> {
    let path = PathBuf::from("./tests/resources/generated/");
    fs::remove_dir_all(&path)?;
    fs::remove_dir_all("./tests/python/.venv")?;
    fs::remove_dir_all("./tests/python/__pycache__")?;
    Ok(())
}

fn build_patch() {
    if let Ok(current_dir) = env::current_dir() {
        startup().expect("Failed to create generated directory");
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
    } else {
        eprintln!("Failed to get current directory");
    }
}

fn install_packages() {
    if let Ok(current_dir) = env::current_dir() {
        startup().expect("Failed to create generated directory");
        let python_dir = current_dir.join(Path::new("./tests/python/"));
        let package_wheel_path =
            "../resources/generated/python/test-patch@0.1.0.0.1.0/dist/test_patch-0.1.0.0.1.0-py3-none-any.whl";
        let _build_venv = exec_cmd(&python_dir, "python3", &["-m", "venv", ".venv"]);
        let _python_stdout = exec_cmd(
            &python_dir,
            "bash",
            &[
                "-e",
                "-c",
                format!("source .venv/bin/activate\npython3 -m pip install --upgrade pip\npip install pytest-asyncio\npip install {} --force-reinstall", package_wheel_path).as_str(),
            ],
        );
        // check that package is installed
        let python_package_check = exec_cmd(
            &python_dir,
            "bash",
            &[
                "-e",
                "-c",
                "source .venv/bin/activate\npython3 -m pip list --local | grep test-patch",
            ],
        );
        assert!(python_package_check.starts_with("test-patch"));
        assert!(python_package_check.ends_with("0.1.0.0.1.0\n"));
    } else {
        eprintln!("Failed to get current directory");
    }
}

fn test_packages() {
    if let Ok(current_dir) = env::current_dir() {
        startup().expect("Failed to create generated directory");
        let python_dir = current_dir.join(Path::new("./tests/python/"));
        if env::var("PATCH_AUTH_TOKEN").is_ok() {
            exec_cmd(
                &python_dir,
                "bash",
                &[
                    "-e",
                    "-c",
                    "source .venv/bin/activate\npytest -s patch_test.py",
                ],
            );
        } else {
            exec_cmd(
                &python_dir,
                "bash",
                &[
                    "-e",
                    "-c",
                    "source .venv/bin/activate\nsops exec-env ../../secrets/dpm.enc.env 'pytest -s patch_test.py'",
                ],
            );
        }
        cleanup().expect("Failed to cleanup generated directory");
    } else {
        eprintln!("Failed to get current directory");
    }
}

#[test]
fn integration_test() {
    build_patch();
    install_packages();
    test_packages();
}
