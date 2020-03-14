use std::env;
use std::process::Command;

fn main() {
    let rustc_command = env::var("RUSTC").unwrap_or_else(|_| String::from("rustc"));
    let rustc_version = Command::new(&rustc_command)
        .arg("--version")
        .arg("--verbose")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap()
        .lines()
        .map(|l| format!("{:?}", l))
        .collect::<Vec<_>>()
        .join(",");

    println!(r#"cargo:rustc-env=RUSTC=[{}]"#, rustc_version);
}
