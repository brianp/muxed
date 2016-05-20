//! The integration suite helpers.

use std::process::Command;
use std::path::{Path, PathBuf};

/// List windows will give details about the active sessions in testing.
/// target: A string represented by the {named_session}:{named_window}
pub fn list_windows(target: &String) -> String {
    let output = Command::new("tmux")
                     .arg("list-windows")
                     .arg("-t")
                     .arg(target)
                     .output()
                     .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    String::from_utf8_lossy(&output.stdout).into_owned()
}

pub fn open_muxed(project: &String, project_root: &Path) -> () {
    println!("root: {}", project_root.display());
    let output = Command::new("./target/debug/muxed")
        .arg("-d")
        .arg("-p")
        .arg(format!("{}", project_root.display()))
        .arg(project)
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}

pub fn kill_session(target: &String) -> () {
    Command::new("tmux")
        .arg("kill-session")
        .arg("-t")
        .arg(target)
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
}

pub struct TmuxSession {
    pub num_of_windows: usize,
}

pub fn session_object(results: &String) -> TmuxSession {
    let lines: Vec<&str> = results.split("\n").collect();
    let (_, window_lines) = lines.split_last().unwrap();
    TmuxSession{num_of_windows: window_lines.len()}
}
