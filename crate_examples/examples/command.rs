use std::process::Command;

fn main() {
    let output = Command::new("ps")
        .arg("-ef")
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
