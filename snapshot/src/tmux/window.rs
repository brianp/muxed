use capture::retrieve_capture;
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use std::io;
use std::process::{Command, Output};
use tmux::pane::Pane;

// Come back and question the accuracy of windows without names
// that have active, or previous window designations.
static NAME_REGEX: &str = r":\s(\w*)[$\*-]?\s+\(";
static ACTIVE_REGEX: &str = r"\s.*(\*)\s";
static LAYOUT_REGEX: &str = r"\s\[layout\s(.*)\]";

// Example format: "2: vim* (1 panes) [layout b5be,173x42,0,0,1]"
static LIST_FORMAT: &str = "'#{window_index}: #{window_name}#{?window_active,*, } (#{window_panes} panes) [layout #{window_layout}]'";

#[derive(Debug, Deserialize)]
pub struct Window {
    pub active: bool,
    pub layout: String,
    pub name: String,
    pub panes: Vec<Pane>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowInner {
    pub active: bool,
    pub layout: String,
    pub panes: Vec<Pane>,
}

impl Window {
    pub fn new<S>(active: bool, layout: S, name: S, panes: Vec<Pane>) -> Window
    where
        S: Into<String>,
    {
        Window {
            active,
            layout: layout.into(),
            name: name.into(),
            panes,
        }
    }

    pub fn from_window(panes: Vec<Pane>, w: Window) -> Window {
        Window::new(w.active, w.layout, w.name, panes)
    }

    pub fn from_line(line: &str) -> Option<Window> {
        let active = retrieve_capture(line, ACTIVE_REGEX).is_some();

        let layout = match retrieve_capture(line, LAYOUT_REGEX) {
            Some(x) => x,
            None => return None,
        };

        let name = match retrieve_capture(line, NAME_REGEX) {
            Some(x) => x,
            None => return None,
        };

        Some(Window::new(active, layout, name, vec![]))
    }

    pub fn window_list(target: &str) -> Result<Output, io::Error> {
        Command::new("tmux")
            .args(&["list-windows", "-t", target, "-F", LIST_FORMAT])
            .output()
    }
}

impl Serialize for Window {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let window = WindowInner {
            active: self.active,
            layout: self.layout.clone(),
            panes: self.panes.clone(),
        };

        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.name, &window)?;
        map.end()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test] // Window with name* representing the active window.
    fn expect_some_from_active_window_line() {
        let line = "2: vim* (1 panes) [layout b5be,173x42,0,0,1]";
        let window = Window::from_line(line);
        assert!(window.is_some())
    }

    #[test] // Window with name- representing the previous active window.
    fn expect_some_from_previous_window_line() {
        let line = "2: vim-  (1 panes) [layout b5be,173x42,0,0,1]";
        let window = Window::from_line(line);
        assert!(window.is_some())
    }

    #[test] // Window with name and no designation.
    fn expect_some_from_window_line() {
        let line = "2: vim  (1 panes) [layout b5be,173x42,0,0,1]";
        let window = Window::from_line(line);
        assert!(window.is_some())
    }

    #[test] // Window with no name and active.
    fn expect_some_from_active_window_blank_name() {
        let line = "2: * (1 panes) [layout b5be,173x42,0,0,1]";
        let window = Window::from_line(line);
        assert!(window.is_some())
    }

    #[test] // Window with no name and the previous active window.
    fn expect_some_from_previous_window_blank_name() {
        let line = "2: - (1 panes) [layout b5be,173x42,0,0,1]";
        let window = Window::from_line(line);
        assert!(window.is_some())
    }

    #[test] // Window with blank name
    fn expect_some_with_blank_name() {
        let line = "2:   (1 panes) [layout b5be,173x42,0,0,1]";
        let window = Window::from_line(line);
        assert!(window.is_some())
    }

    #[test]
    fn expect_none_from_line_missing_name() {
        let line = "2: (1 panes) [layout b5be,173x42,0,0,1]";
        let window = Window::from_line(line);
        assert!(window.is_none())
    }

    #[test]
    fn expect_some_from_line_with_dollar_sign() {
        let line = "2: vim$ (1 panes) [layout b5be,173x42,0,0,1]";
        let window = Window::from_line(line);
        assert!(window.is_some())
    }

    #[test]
    fn expect_active_to_be_true() {
        let line = "2: vim* (1 panes) [layout b5be,173x42,0,0,1]";
        let window = Window::from_line(line).unwrap();
        assert!(window.active)
    }

    #[test]
    fn expect_active_to_be_true_without_name() {
        let line = "2: * (1 panes) [layout b5be,173x42,0,0,1]";
        let window = Window::from_line(line).unwrap();
        assert!(window.active)
    }

    #[test]
    fn expect_name_to_be_vim() {
        let line = "2: vim* (1 panes) [layout b5be,173x42,0,0,1]";
        let window = Window::from_line(line).unwrap();
        assert_eq!(window.name, "vim")
    }

    #[test]
    fn expect_layout_to_match() {
        let line = "2: vim* (1 panes) [layout b5be,173x42,0,0,1]";
        let window = Window::from_line(line).unwrap();
        assert_eq!(window.layout, "b5be,173x42,0,0,1")
    }
}
