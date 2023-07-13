use std::io::Read;
use std::path::Path;
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
