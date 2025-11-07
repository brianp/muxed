//! The structures used to manage commands sent over to tmux.

use std::fmt;

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
    pub fn new<S: AsRef<str> + ToString>(session: S, window: S) -> WindowTarget {
        WindowTarget {
            session: session.as_ref().to_string(),
            window: window.as_ref().to_string(),
            arg_string: format!("{}:{}", session.as_ref(), window.as_ref()),
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
pub struct SessionTarget {
    pub session: String,
    pub arg_string: String,
}

impl SessionTarget {
    pub fn new<S: AsRef<str> + Into<String>>(session: S) -> SessionTarget {
        SessionTarget {
            session: session.as_ref().to_string(),
            arg_string: session.as_ref().to_string(),
        }
    }

    pub fn arg_string(&self) -> &str {
        &self.arg_string
    }
}

impl fmt::Display for SessionTarget {
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
