use std::fs::{self};
use std::path::Path;
use std::path::PathBuf;

use crate::integration_test::target_tester::{exec_cmd, TargetTester};
pub struct Nodejs {}

impl TargetTester for Nodejs {
    fn build_packages(&self, current_dir: &PathBuf, package_ref: &str) {
        let home_dir = current_dir.as_path();
        exec_cmd(
            &home_dir,
            env!("CARGO_BIN_EXE_dpm"),
            &[
                "build-package",
                "-p",
                package_ref,
                "-o",
                "./tests/resources/generated",
                "-y",
                "nodejs",
            ],
        );
        // assert generated directories are not empty
        assert!(
            !fs::read_dir("./tests/resources/generated/nodejs/test-snowflake@0.1.0-0.2.2")
                .map_err(|e| format!("Failed to read directory: {}", e))
                .unwrap()
                .next()
                .is_none()
        );
    }
    fn install_packages(&self, current_dir: &PathBuf) {
        let nodejs_dir = current_dir.join(Path::new("./tests/nodejs/"));
        let package_names = vec!["test-snowflake"];
        for name in package_names {
            let tar_path = format!("../resources/generated/nodejs/{}-0.1.0-0.2.2.tgz", name);
            exec_cmd(&nodejs_dir, "npm", &["install", &tar_path]);
            let ls_stdout = exec_cmd(&nodejs_dir, "npm", &["ls"]);
            assert!(ls_stdout.contains(&name));
        }
    }
    fn test_packages(&self, current_dir: &PathBuf) {
        let nodejs_dir = current_dir.join(Path::new("./tests/nodejs/"));
        exec_cmd(&nodejs_dir, "npm", &["run", "test"]);
    }
    fn cleanup(&self) -> std::io::Result<()> {
        fs::remove_dir_all("./tests/nodejs/node_modules")?;
        fs::remove_file("./tests/nodejs/package-lock.json")?;
        Ok(())
    }
}
