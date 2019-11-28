//use std::path::PathBuf;
//use tmux::window::Window;
use serde::ser::Serialize;
use serde::Serializer;
use capture::retrieve_capture;
use std::io;
use std::path::PathBuf;
use std::process::{Command, Output};

pub mod pid;
pub mod process;

use self::pid::Pid;
use self::process::Process;

static ACTIVE_REGEX: &str = r"\s(\(active\))";
static PATH_REGEX: &str = r"\(Path: (.*)\) \(PID";
static PID_REGEX: &str = r"\(PID: ([0-9]*)\)";

// Example format: "1: [123x14] (Path: /muxed/muxedsnapshot) (PID: 22541) (active)"
static LIST_FORMAT: &str = "'#{pane_index}: [#{pane_width}x#{pane_height}] (Path: #{pane_current_path}) (PID: #{pane_pid}) #{?pane_active,(active), } '";

#[derive(Clone, Debug, Deserialize)]
pub struct Pane {
    pub active: bool,
    pub path: PathBuf,
    pub process: Option<Process>,
}

impl Pane {
    pub fn new(active: bool, path: PathBuf, process: Option<Process>) -> Pane {
        Pane {
            active,
            path,
            process,
        }
    }

    pub fn from_line(line: &str) -> Option<Pane> {
        let active = retrieve_capture(line, ACTIVE_REGEX).is_some();

        let path = match retrieve_capture(line, PATH_REGEX) {
            Some(x) => PathBuf::from(x),
            None => return None,
        };

        let pid = match retrieve_capture(line, PID_REGEX) {
            Some(x) => Pid::new(x),
            None => return None,
        };

        let process = match Process::process_string_from(pid) {
            Ok(x) => Some(Process::new(x)),
            Err(_) => None,
        };

        Some(Pane::new(active, path, process))
    }

    pub fn pane_list(target: &str) -> Result<Output, io::Error> {
        Command::new("tmux")
            .args(&["list-panes", "-t", target, "-F", LIST_FORMAT])
            .output()
    }
}

impl Serialize for Pane {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
    where
        S: Serializer,
    {
        let process_str = match self.process.clone() {
            Some(process) => process.process,
            None => "".to_string(),
        };

        serializer.serialize_str(process_str.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn expect_active_to_be_false() {
        let line = "2: [123x14] (Path: /muxed-project/muxed) (PID: 22541)  ";
        let pane = Pane::from_line(line).unwrap();
        assert!(!pane.active)
    }

    #[test]
    fn expect_active_to_be_true() {
        let line = "1: [123x14] (Path: /muxed-project/muxed) (PID: 22541) (active)";
        let pane = Pane::from_line(line).unwrap();
        assert!(pane.active)
    }

    #[test]
    fn expect_pane_to_match() {
        let line = "1: [123x14] (Path: /muxed/muxedsnapshot) (PID: 22541) (active)";
        let pane = Pane::from_line(line).unwrap();
        assert_eq!(pane.path, PathBuf::from("/muxed/muxedsnapshot"));
    }
}
