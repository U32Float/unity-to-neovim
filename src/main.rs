use std::process::Command;

fn main() {
    let args = std::env::args();
    let files: Vec<String> = args.into_iter().skip(1).collect();
    Command::new("nvr")
        .arg("--remote")
        .args(files)
        .output()
        .expect("Failed to send to neovim");
}
