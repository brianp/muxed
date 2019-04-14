//! The structures used to manage commands sent over to tmux.

use std::path::PathBuf;
use std::str;

pub trait Command {
    fn call<S>(&self) -> Vec<&str>;
}

/// The Session command is used to fire up a new daemonized session in tmux.
/// `name`: The Name of a named tmux session.
/// `window_name`: The Name of the first window.
/// `root_path`: The root directory for the tmux session.
#[derive(Debug, Clone)]
pub struct Session {
    pub name: String,
    pub window_name: String,
    pub root_path: Option<PathBuf>,
}

// TODO: Real logic exists here. Test it!
impl Command for Session {
    fn call<S>(&self) -> Vec<&str> {
        let command: Vec<&str> = vec!["new", "-d", "-s", &self.name, "-n", &self.window_name];

        match self.root_path.as_ref() {
            Some(path) => [&command[..], &["-c", path.to_str().unwrap()]].concat(),
            None => command,
        }
    }
}

/// The Window command is used to identify every new window opened in the tmux
/// session.
/// `session_name`: The name of the session.
/// `name`: The named window to be opened.
/// `path`: An `Option<PathBuf>` containing a possible root directory passed to the
/// `-c` arguement.
#[derive(Debug, Clone)]
pub struct Window {
    pub session_name: String,
    pub name: String,
    pub path: Option<PathBuf>,
}

impl Command for Window {
    fn call<S>(&self) -> Vec<&str> {
        let command: Vec<&str> = vec!["new-window", "-t", &self.session_name, "-n", &self.name];

        match self.path.as_ref() {
            Some(path) => [&command[..], &["-c", path.to_str().unwrap()]].concat(),
            None => command,
        }
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
        vec!["split-window", "-t", &self.target]
    }
}

/// The Layout command calls select-layout with a specific pre-defined tmux
/// layout option see `tmux select-layout --help` for more options.
/// `target`: The target window. In the format `{session}:{window}.{paneIndex}`.
/// `layout`: The type of layout. ex `main-horizontal`.
#[derive(Debug, Clone)]
pub struct Layout {
    pub target: String,
    pub layout: String,
}

impl Command for Layout {
    fn call<S>(&self) -> Vec<&str> {
        vec!["select-layout", "-t", &self.target, &self.layout]
    }
}

/// A generic `SendKeys` command used to send "typed" commands to tmux. This is
/// used to initialize processes or tasks in specific window. Such as starting log
/// tails or running servers.
/// target: The target window. In the format `{session}:{window}.{paneIndex}`.
/// exec: The cli command to be run. ex. `tail -f logs/development.log`.
#[derive(Debug, Clone)]
pub struct SendKeys {
    pub target: String,
    pub exec: String,
}

impl Command for SendKeys {
    fn call<S>(&self) -> Vec<&str> {
        vec!["send-keys", "-t", &self.target, &self.exec, "KPEnter"]
    }
}

/// Used to attach to the daemonized session.
/// name: The named session to attach too.
#[derive(Debug, Clone)]
pub struct Attach {
    pub name: String,
}

impl Command for Attach {
    fn call<S>(&self) -> Vec<&str> {
        // No-op!
        vec![]
    }
}

/// Used to move focus back to the first window.
/// target: The target window. In the format `{session}:{window}`.
#[derive(Debug, Clone)]
pub struct SelectWindow {
    pub target: String,
}

impl Command for SelectWindow {
    fn call<S>(&self) -> Vec<&str> {
        vec!["select-window", "-t", &self.target]
    }
}

/// Used to move focus back to the top pane.
/// target: The target pane. In the format `{session}:{window}.{pane-target}`.
#[derive(Debug, Clone)]
pub struct SelectPane {
    pub target: String,
}

impl Command for SelectPane {
    fn call<S>(&self) -> Vec<&str> {
        vec!["select-pane", "-t", &self.target]
    }
}

/// Used for executing the `pre` option to execute commands before building the
/// tmux session.
/// exec: The command to execute
#[derive(Debug, Clone)]
pub struct Pre {
    pub exec: String,
}

impl Command for Pre {
    fn call<S>(&self) -> Vec<&str> {
        // No-op!
        vec![]
    }
}

/// The Command enum. Commands represent the series of commands sent to the
/// running tmux process to build a users env. This is an enum to support
/// containing all the commands that require running in a single Vec. This
/// allows a simple process of first in, first out command execution.
#[derive(Debug, Clone)]
pub enum Commands {
    Attach(Attach),
    Layout(Layout),
    Pre(Pre),
    SelectPane(SelectPane),
    SelectWindow(SelectWindow),
    SendKeys(SendKeys),
    Session(Session),
    Split(Split),
    Window(Window),
}
