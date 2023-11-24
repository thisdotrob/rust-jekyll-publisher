use std::process::Command;
use std::str;

fn setup() {
    let stdout = run_command("git remote -v show");
    let (remote_name, remote_addr) = parse_git_remote_details(stdout);

    run_command("echo _site >> .gitignore");
    run_command("jekyll build");
    run_command("git checkout -b sources");
    run_command("git add -A");
    run_command("git commit -m \"Jekyll base sources\"");
    run_command("git push origin sources");

    run_command("cd _site && touch .nojekyll");
    run_command("cd _site && git init");

    let remote_add_cmd = format!(
        "cd _site && git remote add {} {}",
        &remote_name, &remote_addr
    );
    run_command(&remote_add_cmd);

    run_command("cd _site && git add -A");
    run_command("cd _site && git commit -m \"Initial publish\"");

    let stdout = run_command("cd _site && git branch --show-current");
    let branch_name = parse_git_branch_name(stdout);

    let push_cmd = format!(
        "cd _site && git push -f -u {} {}",
        &remote_name, &branch_name
    );
    run_command(&push_cmd);

    println!("\nFinished setup.");
}

fn publish() {
    let stdout = run_command("git status");
    panic_if_uncommitted_changes(stdout);
    run_command("cd _site && git add -A");
    run_command("cd _site && git commit -m \"Publish\"");
    run_command("cd _site && git push");

    println!("\nFinished publishing.");
}

fn run_command(command: &str) -> Vec<u8> {
    let error_msg = format!("failed to run `{}`", command);
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect(&error_msg);
    return output.stdout;
}

fn parse_git_remote_details(stdout: Vec<u8>) -> (String, String) {
    let mut lines = stdout
        .split(|&b| b == b'\n' || b == b'\t' || b == b' ')
        .map(|line| str::from_utf8(line).unwrap());
    let remote_name = lines.next().unwrap();
    let remote_addr = lines.next().unwrap();
    return (String::from(remote_name), String::from(remote_addr));
}

fn parse_git_branch_name(stdout: Vec<u8>) -> String {
    let branch_name = str::from_utf8(&stdout).unwrap();
    let branch_name = branch_name.trim();
    return String::from(branch_name);
}

fn panic_if_uncommitted_changes(stdout: Vec<u8>) {
    let mut lines = stdout
        .split(|&b| b == b'\n')
        .map(|line| str::from_utf8(line).unwrap());
    lines.next().unwrap();
    let git_status_message = lines.next().unwrap();
    if git_status_message != "nothing to commit, working tree clean" {
        panic!("Can't publish when there are uncommitted changes");
    }
}

fn main() {
    let arg = std::env::args()
        .nth(1)
        .expect("no argument given, should be either 'setup' or 'publish'");

    if arg == "setup" {
        setup();
    } else {
        publish();
    }
}
