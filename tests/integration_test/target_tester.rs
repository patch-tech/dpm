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
