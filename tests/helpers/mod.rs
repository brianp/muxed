//! The integration suite helpers.

use regex::Regex;
use std::process::Command;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

/// List windows will give details about the active sessions in testing.
/// target: A string represented by the {named_session}:{named_window}
/// tmux list-windows -t TARGET -F '#{window_index}: #{window_name}#{?window_active,*, } (#{window_panes} panes) (Dir: #{pane_current_path}) (Session: #{session_name})'
pub fn list_windows(target: &String) -> String {
    let output = Command::new("tmux")
                     .arg("list-windows")
                     .arg("-t")
                     .arg(target)
                     .arg("-F")
                     .arg("'#{window_index}: #{window_name}#{?window_active,*, } (#{window_panes} panes) (Dir: #{pane_current_path}) (Session: #{session_name})'")
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

pub fn kill_session(target: &str) -> () {
    Command::new("tmux")
        .arg("kill-session")
        .arg("-t")
        .arg(target)
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
}

pub fn send_keys(target: &String, exec: &String) -> () {
    Command::new("tmux")
        .arg("send-keys")
        .arg("-t")
        .arg(target)
        .arg(exec)
        .arg("KPEnter")
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
}

pub fn wait_on(file: &PathBuf) -> () {
    while !file.exists() {
        // Wait increased from 10 to 750 due to the pre_window tests.
        sleep(Duration::from_millis(750));
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SessionValue {
    Usize(usize),
    String(String),
    Empty,
}

impl SessionValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            &SessionValue::String(ref s) => Some(s),
            _ => None
        }
    }

    pub fn as_usize(&self) -> Option<usize> {
        match self {
            &SessionValue::Usize(s) => Some(s),
            _ => None
        }
    }
}

#[derive(Debug)]
pub struct TmuxSession {
    pub num_of_windows: usize,
    pub windows: HashMap<String, WindowValues>,
    pub window_active: SessionValue,
    pub name: SessionValue,
}

#[derive(Debug, Clone)]
pub struct WindowValues {
    pub panes: SessionValue,
    pub pane_current_path: SessionValue,
}

static NAME_REGEX:              &'static str = r"\(Session: (.*)\)";
static WINDOW_NAME_REGEX:       &'static str = r":\s(\w*)[$\*-]?\s+\(";
static WINDOW_ACTIVE_REGEX:     &'static str = r"\s(.*)\*";
static PANE_CURRENT_PATH_REGEX: &'static str = r"\(Dir: (.*)\) ";
static PANES_COUNT_REGEX:       &'static str = r"\((\d*) panes\)";

impl TmuxSession {
    pub fn from_string(results: &str) -> TmuxSession {
        let window_name = Regex::new(WINDOW_NAME_REGEX).unwrap();

        let lines: Vec<&str> = results.split('\n').collect();
        let (_, window_lines) = lines.split_last().unwrap();

        let mut windows: HashMap<String, WindowValues> = HashMap::new();

        for line in window_lines {
            let cap = window_name.captures(line).unwrap();
            let name = cap.get(1).unwrap().as_str();

            let win_val = WindowValues{
                panes: TmuxSession::count_panes(line),
                pane_current_path: TmuxSession::retrieve_capture(line, PANE_CURRENT_PATH_REGEX).unwrap_or(SessionValue::Empty)
            };

            windows.insert(name.to_string(), win_val.clone());
        }

        TmuxSession {
          num_of_windows: window_lines.len(),
          windows: windows,
          window_active: TmuxSession::retrieve_capture(window_lines[0], WINDOW_ACTIVE_REGEX).unwrap_or(SessionValue::Empty),
          name: TmuxSession::retrieve_capture(window_lines[0], NAME_REGEX).unwrap_or(SessionValue::Empty)
        }
    }

    pub fn retrieve_capture(line: &str, pattern: &str) -> Result<SessionValue, String> {
        let reg = Regex::new(pattern).unwrap();

        if let Some(caps) = reg.captures(line) {
            return match caps.get(1) {
               Some(x) => Ok(SessionValue::String(x.as_str().to_string())),
               None    => Err("No capture".to_string())
            };
        };

        Err("No capture".to_string())
    }

    pub fn count_panes(line: &str) -> SessionValue {
        let panes = Regex::new(PANES_COUNT_REGEX).unwrap();
        let mut num: &str = "";

        for cap in panes.captures_iter(line) {
            num = match cap.get(1) {
                Some(x) => x.as_str(),
                None    => "0"
            }
        }

        SessionValue::Usize(usize::from_str(num).unwrap())
    }
}

#[test]
fn count_panes_returns_two() {
    let num = TmuxSession::count_panes("1: ssh  (2 panes) (Dir: /Projects/muxed) (session: muxed)");
    assert_eq!(num.as_usize().unwrap(), 2)
}

#[test]
fn count_panes_returns_one() {
    let num = TmuxSession::count_panes("1: ssh  (1 panes) (Dir: /Projects/muxed) (session: muxed)");
    assert_eq!(num.as_usize().unwrap(), 1)
}

#[test]
fn parses_with_trailing_whitespace_after_window_name() {
    let config = "1: ssh  (2 panes) (Dir: /Projects/muxed) (Session: muxed)\n";
    let session = TmuxSession::from_string(&config.to_string());
    let panes = session.windows["ssh"].panes.as_usize().unwrap();
    assert_eq!(session.num_of_windows, 1);
    assert_eq!(panes, 2)
}

#[test]
fn parses_with_previous_flag() {
    let config = "1: ssh- (2 panes) (Dir: /Projects/muxed) (Session: muxed)\n";
    let session = TmuxSession::from_string(&config.to_string());
    let panes = session.windows["ssh"].panes.as_usize().unwrap();
    assert_eq!(session.num_of_windows, 1);
    assert_eq!(panes, 2)
}

#[test]
fn parses_with_dollar_sign_flag() {
    let config = "1: ssh$ (2 panes) (Dir: /Projects/muxed) (Session: muxed)\n";
    let session = TmuxSession::from_string(&config.to_string());
    let panes = session.windows["ssh"].panes.as_usize().unwrap();
    assert_eq!(session.num_of_windows, 1);
    assert_eq!(panes, 2)
}

#[test]
fn parses_with_window_flag() {
    let config = "1: ssh* (2 panes) (Dir: /Projects/muxed) (Session: muxed)\n";
    let session = TmuxSession::from_string(&config.to_string());
    let panes = session.windows["ssh"].panes.as_usize().unwrap();
    assert_eq!(session.num_of_windows, 1);
    assert_eq!(panes, 2)
}

#[test]
fn count_three_windows() {
    let config = "1: ssh  (1 panes) (Dir: /Projects/muxed) (Session: muxed)
                  2: vim- (1 panes) (Dir: /Projects/muxed) (Session: muxed)
                  3: bash* (2 panes) (Dir: /Projects/muxed) (Session: muxed)\n";
    let num = TmuxSession::from_string(&config.to_string()).num_of_windows;
    assert_eq!(num, 3)
}

#[test]
fn count_four_total_panes() {
    let config = "1: ssh  (1 panes) (Dir: /Projects/muxed) (Session: muxed)
                  2: vim- (1 panes) (Dir: /Projects/muxed) (Session: muxed)
                  3: bash* (2 panes) (Dir: /Projects/muxed) (Session: muxed)\n";
    let session = TmuxSession::from_string(&config.to_string());
    let num = session.windows["ssh"].panes.as_usize().unwrap();
    let num1 = session.windows["vim"].panes.as_usize().unwrap();
    let num2 = session.windows["bash"].panes.as_usize().unwrap();
    assert_eq!(num, 1);
    assert_eq!(num1, 1);
    assert_eq!(num2, 2)
}

#[test]
fn expect_ok_session_name_capture() {
    let line = "2: vim1* (1 panes) (Dir: /Projects/muxed) (Session: muxed)";
    let result = TmuxSession::retrieve_capture(line, NAME_REGEX);
    assert!(result.is_ok())
}

#[test]
fn expect_err_session_name_capture() {
    let line = "2: vim1* (1 panes) (Dir: /Projects/muxed) (Session:)";
    let result = TmuxSession::retrieve_capture(line, NAME_REGEX);
    assert!(result.is_err())
}

#[test]
fn expect_muxed_to_be_captured_as_session() {
    let line = "2: vim1* (1 panes) (Dir: /Projects/muxed) (Session: muxed)";
    let result = TmuxSession::retrieve_capture(line, NAME_REGEX).unwrap();
    assert_eq!(result.as_str().unwrap(), "muxed")
}

#[test]
fn expect_ok_window_name_capture() {
    let line = "2: vim1* (1 panes) vim2* (Dir: /Projects/muxed) (Session: muxed)";
    let result = TmuxSession::retrieve_capture(line, WINDOW_NAME_REGEX);
    assert!(result.is_ok())
}

#[test]
fn expect_err_window_name_capture() {
    let line = "2: (1 panes) vim2* (Dir: /Projects/muxed) (Session: muxed)";
    let result = TmuxSession::retrieve_capture(line, WINDOW_NAME_REGEX);
    assert!(result.is_err())
}

#[test]
fn expect_vim1_window_name_capture() {
    let line = "2: vim1* (1 panes) vim2* (Dir: /Projects/muxed) (Session: muxed)";
    let result = TmuxSession::retrieve_capture(line, WINDOW_NAME_REGEX).unwrap();
    assert_eq!(result.as_str().unwrap(), "vim1")
}

#[test]
fn expect_ok_active_window_capture() {
    let line = "2: vim* (1 panes) (Dir: /Projects/muxed) (Session: muxed)";
    let result = TmuxSession::retrieve_capture(line, WINDOW_ACTIVE_REGEX);
    assert!(result.is_ok())
}

#[test]
fn expect_err_active_window_capture() {
    let line = "2: vim (1 panes) (Dir: /Projects/muxed) (Session: muxed)";
    let result = TmuxSession::retrieve_capture(line, WINDOW_ACTIVE_REGEX);
    assert!(result.is_err())
}

#[test]
fn expect_vim_active_window_capture() {
    let line = "2: vim* (1 panes) (Dir: /Projects/muxed) (Session: muxed)";
    let result = TmuxSession::retrieve_capture(line, WINDOW_ACTIVE_REGEX).unwrap();
    assert_eq!(result.as_str().unwrap(), "vim")
}
