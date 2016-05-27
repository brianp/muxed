//! The integration suite helpers.

use regex::Regex;
use std::process::Command;
use std::path::Path;
use std::collections::HashMap;
use std::str::FromStr;

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

#[derive(Debug)]
pub struct TmuxSession {
    pub num_of_windows: usize,
    pub windows: HashMap<String, HashMap<String, usize>>
}

impl TmuxSession {
    pub fn from_string(results: &String) -> TmuxSession {
        let window_name = Regex::new(r":\s(\w*)[$\*-]?\s+\(").unwrap();

        let lines: Vec<&str> = results.split("\n").collect();
        let (_, window_lines) = lines.split_last().unwrap();

        let mut windows: HashMap<String, HashMap<String, usize>> = HashMap::new();
        let mut h = HashMap::new();

        for line in window_lines {
            for cap in window_name.captures_iter(line) {
                let name = cap.at(1).unwrap();
                h.insert("Panes".to_string(), TmuxSession::count_panes(line));
                windows.insert(name.to_string(), h.clone());
            }
        }

        TmuxSession {
          num_of_windows: window_lines.len(),
          windows: windows
        }
    }

    pub fn count_panes(line: &str) -> usize {
        let panes = Regex::new(r"\((\d*) panes\)").unwrap();
        let mut num: &str = "";

        for cap in panes.captures_iter(line) {
            num = cap.at(1).unwrap_or("0");
        }

        usize::from_str(num).unwrap()
    }
}

#[test]
fn count_panes_returns_two() {
    let num = TmuxSession::count_panes("1: ssh (2 panes) [173x42] [layout b5bd,173x42,0,0,0] @0");
    assert_eq!(num, 2)
}

#[test]
fn count_panes_returns_one() {
    let num = TmuxSession::count_panes("1: ssh (1 panes) [173x42] [layout b5bd,173x42,0,0,0] @0");
    assert_eq!(num, 1)
}

#[test]
fn parses_with_trailing_whitespace_after_window_name() {
    let config = "1: ssh  (2 panes) [173x42] [layout b5bd,173x42,0,0,0] @0\n";
    let session = TmuxSession::from_string(&config.to_string());
    let panes = session.windows.get("ssh").unwrap().get("Panes").unwrap().to_owned();
    assert_eq!(session.num_of_windows, 1);
    assert_eq!(panes, 2)
}

#[test]
fn parses_with_previous_flag() {
    let config = "1: ssh- (2 panes) [173x42] [layout b5bd,173x42,0,0,0] @0\n";
    let session = TmuxSession::from_string(&config.to_string());
    let panes = session.windows.get("ssh").unwrap().get("Panes").unwrap().to_owned();
    assert_eq!(session.num_of_windows, 1);
    assert_eq!(panes, 2)
}

#[test]
fn parses_with_dollar_sign_flag() {
    let config = "1: ssh$ (2 panes) [173x42] [layout b5bd,173x42,0,0,0] @0\n";
    let session = TmuxSession::from_string(&config.to_string());
    let panes = session.windows.get("ssh").unwrap().get("Panes").unwrap().to_owned();
    assert_eq!(session.num_of_windows, 1);
    assert_eq!(panes, 2)
}

#[test]
fn parses_with_window_flag() {
    let config = "1: ssh* (2 panes) [173x42] [layout b5bd,173x42,0,0,0] @0\n";
    let session = TmuxSession::from_string(&config.to_string());
    let panes = session.windows.get("ssh").unwrap().get("Panes").unwrap().to_owned();
    assert_eq!(session.num_of_windows, 1);
    assert_eq!(panes, 2)
}

#[test]
fn count_three_windows() {
    let config = "1: ssh  (1 panes) [173x42] [layout b5bd,173x42,0,0,0] @0
                  2: vim- (1 panes) [173x42] [layout b5be,173x42,0,0,1] @1
                  3: bash* (2 panes) [173x42] [layout b5bf,173x42,0,0,2] @2 (active)\n";
    let num = TmuxSession::from_string(&config.to_string()).num_of_windows;
    assert_eq!(num, 3)
}

#[test]
fn count_four_total_panes() {
    let config = "1: ssh  (1 panes) [173x42] [layout b5bd,173x42,0,0,0] @0
                  2: vim- (1 panes) [173x42] [layout b5be,173x42,0,0,1] @1
                  3: bash* (2 panes) [173x42] [layout b5bf,173x42,0,0,2] @2 (active)\n";
    let session = TmuxSession::from_string(&config.to_string());
    let num = session.windows.get("ssh").unwrap().get("Panes").unwrap().to_owned();
    let num1 = session.windows.get("vim").unwrap().get("Panes").unwrap().to_owned();
    let num2 = session.windows.get("bash").unwrap().get("Panes").unwrap().to_owned();
    assert_eq!(num, 1);
    assert_eq!(num1, 1);
    assert_eq!(num2, 2)
}
