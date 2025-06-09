use std::path::PathBuf;
use std::process::Command;

pub fn get_git_info(path: &PathBuf) -> String {
    let output = Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--pretty=format:'%an'")
        .arg("--")
        .arg(path)
        .output()
        .unwrap();
    let username = String::from_utf8(output.stdout).unwrap().trim().to_string();
    let output = Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--pretty=format:'%ae'")
        .arg("--")
        .arg(path)
        .output()
        .unwrap();
    let email = String::from_utf8(output.stdout).unwrap().trim().to_string();
    let output = Command::new("git")
        .arg("log")
        .arg("-1")
        .arg("--pretty=format:'%ai'")
        .arg("--")
        .arg(path)
        .output()
        .unwrap();
    let date = String::from_utf8(output.stdout).unwrap().trim().to_string();
    format!(
        r#"last commit at {}
last commit from {}
last commit email {}

"#,
        date, username, email
    )
}
