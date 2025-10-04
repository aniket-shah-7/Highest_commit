use std::process::{Command, Stdio};
use std::str;

fn run_git(args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("failed to execute git command");
    if !output.status.success() {
        panic!("git command failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn main() {
    let start = 1;
    let end = 100_000_000; // Changed to 10,000 commits

    let head = run_git(&["rev-parse", "HEAD"]);
    let tree = run_git(&["rev-parse", "HEAD^{tree}"]);
    let mut parent = head;

    let branch = run_git(&["rev-parse", "--abbrev-ref", "HEAD"]);
    for i in start..=end {
        let msg = format!("empty commit #{}", i);
        let commit = run_git(&["commit-tree", &tree, "-p", &parent, "-m", &msg]);
        parent = commit;
        if i % 10_000 == 0 {
            println!("Created {} commits...", i);
        }
        if i % 50_000 == 0 {
            // Update branch and push every 100,000 commits
            run_git(&["update-ref", &format!("refs/heads/{}", branch), &parent]);
            run_git(&["push", "-f"]);
            println!("Auto-pushed at {} commits.", i);
        }
    }
    // Final update and push at the end
    run_git(&["update-ref", &format!("refs/heads/{}", branch), &parent]);
    run_git(&["push", "-f"]);
    println!("Pushed all {} commits using commit-tree.", end);
}
