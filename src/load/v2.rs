//! The structures used to manage commands sent over to tmux.

use std::path::Path;

pub trait Command {
    fn call<S>(&self) -> Vec<&str>;
}

/// The Session command is used to fire up a new daemonized session in tmux.
/// `name`: The Name of a named tmux session.
/// `window_name`: The Name of the first window.
/// `root_path`: The root directory for the tmux session.
pub struct Session {
    pub name: String,
    pub window_name: String,
    pub root_path: Path
}

impl Command for Session {
    fn call<S>(&self) -> Vec<&str> {
        vec!("new", "-d", "-s", &self.name, "-n", &self.window_name, "-c", &self.root_path.to_str().unwrap())
    }
}

/// The Window command is used to identify every new window opened in the tmux
/// session.
/// `session_name`: The name of the session.
/// `name`: The named window to be opened.
/// `path`: An `Option<String>` containing a possible root directory passed to the
/// `-c` arguement.
#[derive(Debug, Clone)]
pub struct Window {
    pub session_name: String,
    pub name: String,
    // pub path: Path
}

impl Command for Window {
    fn call<S>(&self) -> Vec<&str> {
        vec!("new-window", "-t", &self.session_name, "-n", &self.name)
    }
}

/// The Split is used to call split-window on a particular window in the
/// session.
/// `target`: The target window. In the format `{session}:{window}.{paneIndex}`.
#[derive(Debug, Clone)]
pub struct Split {
    pub target: String,
}

impl Command for Split {
    fn call<S>(&self) -> Vec<&str> {
        vec!("split-window", "-t", &self.target)
    }
}
