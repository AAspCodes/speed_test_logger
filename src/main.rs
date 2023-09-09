use std::process::Command;

fn main() {
    let result = run_cmd(String::from("ls"));
    println!("{}", result)
}

fn run_cmd(cmd: String) -> String {
    let output = Command::new("sh")
        .args(["-c", cmd.as_str()])
        .output()
        .expect("failed to execute process");

    let result = String::from_utf8(output.stdout).unwrap();
    return result;
}
