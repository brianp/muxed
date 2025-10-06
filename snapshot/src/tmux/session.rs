use crate::tmux::window::Window;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub name: String,
    pub windows: Vec<Window>,
}

impl Session {
    pub fn new<S>(name: S, windows: Vec<Window>) -> Session
    where
        S: Into<String>,
    {
        Session {
            name: name.into(),
            windows,
        }
    }

    pub fn find_window(&self, name: &str) -> Option<&Window> {
        self.windows.iter().find(|&w| w.name == name)
    }
}
