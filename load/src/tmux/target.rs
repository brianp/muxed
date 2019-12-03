//! The structures used to manage commands sent over to tmux.

use std::fmt;
use std::usize;

/// A targeted pane for a tmux session
#[derive(Debug, Clone)]
pub struct PaneTarget {
    pub session: String,
    pub window: String,
    pub pane_index: usize,
    pub arg_string: String,
}

impl PaneTarget {
    pub fn new(session: &str, window: &str, pane_index: usize) -> PaneTarget {
        PaneTarget {
            session: session.to_string(),
            window: window.to_string(),
            pane_index,
            arg_string: format!("{}:{}.{}", session, window, pane_index),
        }
    }
}

impl fmt::Display for PaneTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.arg_string)
    }
}

/// A targeted window for a tmux session
#[derive(Debug, Clone)]
pub struct WindowTarget {
    pub session: String,
    pub window: String,
    pub arg_string: String,
}

impl WindowTarget {
    pub fn new(session: &str, window: &str) -> WindowTarget {
        WindowTarget {
            session: session.to_string(),
            window: window.to_string(),
            arg_string: format!("{}:{}", session, window),
        }
    }
}

impl fmt::Display for WindowTarget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.arg_string)
    }
}

/// A targeted session for tmux
#[derive(Debug, Clone)]
pub struct SessionTarget<'a> {
    pub session: &'a str,
    pub arg_string: &'a str,
}

impl<'a> SessionTarget<'a> {
    pub fn new(session: &'a str) -> SessionTarget<'a> {
        SessionTarget {
            session,
            arg_string: session,
        }
    }
}

impl<'a> fmt::Display for SessionTarget<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.session)
    }
}

#[derive(Debug, Clone)]
pub enum Target {
    PaneTarget(PaneTarget),
    WindowTarget(WindowTarget),
}

impl Target {
    pub fn arg_string(&self) -> &str {
        match *self {
            Target::PaneTarget(ref c) => &c.arg_string,
            Target::WindowTarget(ref c) => &c.arg_string,
        }
    }
}
