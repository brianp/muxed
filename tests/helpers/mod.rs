//! The integration suite helpers.

use std::process::Command;
use std::path::PathBuf;
use std::env::home_dir;

pub fn homedir() -> Result<PathBuf, String>{
    match home_dir() {
        Some(dir) => Ok(dir),
        None      => Err(String::from("We couldn't find your home directory."))
    }
}

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

pub fn open_muxed(project: &String) -> () {
    Command::new("./target/debug/muxed")
        .arg("-d")
        .arg(format!("{}", project))
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
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
