use crate::entity::Entity;
use crate::error::SnapshotError;
use common::tmux::{Pane, Session, Target, Window};
use std::collections::BTreeMap;
use std::process::Output;
use common::DEBUG;

type Result<T> = std::result::Result<T, SnapshotError>;

#[derive(Debug)]
pub struct SessionOutput {
    pub output: Output,
    pub target: Target,
}

impl TryFrom<SessionOutput> for Session {
    type Error = SnapshotError;

    fn try_from(session: SessionOutput) -> Result<Self> {
        let mut windows: BTreeMap<usize, Window> = BTreeMap::new();

        for line in String::from_utf8_lossy(&session.output.stdout).lines() {
            if DEBUG.load() {
                println!("line {}", line);
            }

            let entity: Entity = serde_json::from_str(line)?;

            // Windows are always listed first. So we can assume all windows are present when we parse panes.
            match &entity {
                w @ Entity::Window { .. } => {
                    let window = Window::try_from(w)?;
                    windows.insert(w.index(), window);
                }
                p @ Entity::Pane { window_index, .. } => {
                    let pane = Pane::try_from(p)?;
                    if let Some(window) = windows.get_mut(window_index) {
                        window.panes.push(pane)
                    }
                }
            }
        }

        Ok(Session {
            name: Some(session.target.combined.clone()),
            pre: None,
            pre_window: None,
            root: None,
            windows: windows.into_values().collect(),
            target: Some(session.target),
            daemonize: None,
            config: None,
        })
    }
}

#[cfg(test)]
mod test {
    use std::os::unix::process::ExitStatusExt;
    use super::*;
    use std::process::{Output};

    fn create_mock_output(stdout: &str, stderr: &str, success: bool) -> Output {
        Output {
            status: if success {
                std::process::ExitStatus::from_raw(0)
            } else {
                std::process::ExitStatus::from_raw(1)
            },
            stdout: stdout.as_bytes().to_vec(),
            stderr: stderr.as_bytes().to_vec(),
        }
    }

    #[test]
    fn test_session_from_empty_output() {
        let target = Target::new("test-session", None, None);
        let session_output = SessionOutput {
            output: create_mock_output("", "", true),
            target: target.clone(),
        };
        let session = Session::try_from(session_output).unwrap();
        assert_eq!(session.name, Some("test-session".to_string()));
        assert_eq!(session.windows.len(), 0);
        assert_eq!(session.target, Some(target));
    }

    #[test]
    fn test_session_from_single_window() {
        let target = Target::new("test-session", None, None);
        let json_output = r#"{"type":"window","session":"test-session","index":0,"name":"main","active":1,"layout":"even-horizontal"}
"#;

        let session_output = SessionOutput {
            output: create_mock_output(json_output, "", true),
            target: target.clone(),
        };

        let session = Session::try_from(session_output).unwrap();
        assert_eq!(session.name, Some("test-session".to_string()));
        assert_eq!(session.windows.len(), 1);

        let window = &session.windows[0];
        assert_eq!(window.name, "main");
        assert!(window.active);
        assert_eq!(window.layout, Some("even-horizontal".to_string()));
        assert_eq!(window.panes.len(), 0);
    }

    #[test]
    fn test_session_from_window_with_panes() {
        let target = Target::new("test-session", None, None);
        let json_output = r#"{"type":"window","session":"test-session","index":0,"name":"main","active":1,"layout":"even-horizontal"}
{"type":"pane","session":"test-session","window_index":0,"index":0,"active":1,"path":"/home/user","pid":12345}
{"type":"pane","session":"test-session","window_index":0,"index":1,"active":0,"path":"/tmp","pid":12346}
"#;

        let session_output = SessionOutput {
            output: create_mock_output(json_output, "", true),
            target: target.clone(),
        };

        let session = Session::try_from(session_output).unwrap();
        assert_eq!(session.windows.len(), 1);

        let window = &session.windows[0];
        assert_eq!(window.name, "main");
        assert_eq!(window.panes.len(), 2);

        let pane1 = &window.panes[0];
        assert!(pane1.active);
        assert_eq!(pane1.path, Some(std::path::PathBuf::from("/home/user")));

        let pane2 = &window.panes[1];
        assert!(!pane2.active);
        assert_eq!(pane2.path, Some(std::path::PathBuf::from("/tmp")));
    }

    #[test]
    fn test_session_from_multiple_windows() {
        let target = Target::new("test-session", None, None);
        let json_output = r#"{"type":"window","session":"test-session","index":0,"name":"main","active":1,"layout":"even-horizontal"}
{"type":"window","session":"test-session","index":1,"name":"editor","active":0,"layout":"main-vertical"}
{"type":"pane","session":"test-session","window_index":0,"index":0,"active":1,"path":"/home/user","pid":12345}
{"type":"pane","session":"test-session","window_index":1,"index":0,"active":0,"path":"/project","pid":12347}
"#;

        let session_output = SessionOutput {
            output: create_mock_output(json_output, "", true),
            target: target.clone(),
        };

        let session = Session::try_from(session_output).unwrap();
        assert_eq!(session.windows.len(), 2);

        // Windows should be ordered by index in BTreeMap
        let window0 = &session.windows[0];
        assert_eq!(window0.name, "main");
        assert!(window0.active);
        assert_eq!(window0.panes.len(), 1);

        let window1 = &session.windows[1];
        assert_eq!(window1.name, "editor");
        assert!(!window1.active);
        assert_eq!(window1.layout, Some("main-vertical".to_string()));
        assert_eq!(window1.panes.len(), 1);
    }

    #[test]
    fn test_session_from_pane_without_window() {
        let target = Target::new("test-session", None, None);
        // Pane references window_index 5 but no window with index 5 exists
        let json_output = r#"{"type":"window","session":"test-session","index":0,"name":"main","active":1,"layout":"even-horizontal"}
{"type":"pane","session":"test-session","window_index":5,"index":0,"active":1,"path":"/home/user","pid":12345}
"#;

        let session_output = SessionOutput {
            output: create_mock_output(json_output, "", true),
            target: target.clone(),
        };

        let session = Session::try_from(session_output).unwrap();
        assert_eq!(session.windows.len(), 1);

        // The pane should be silently ignored since its window doesn't exist
        let window = &session.windows[0];
        assert_eq!(window.name, "main");
        assert_eq!(window.panes.len(), 0);
    }

    #[test]
    fn test_session_from_invalid_json() {
        let target = Target::new("test-session", None, None);
        let invalid_json = "not valid json\n";

        let session_output = SessionOutput {
            output: create_mock_output(invalid_json, "", true),
            target: target.clone(),
        };

        let result = Session::try_from(session_output);
        assert!(result.is_err());
    }

    #[test]
    fn test_session_from_mixed_valid_invalid_json() {
        let target = Target::new("test-session", None, None);
        let mixed_output = r#"{"type":"window","session":"test-session","index":0,"name":"main","active":1,"layout":"even-horizontal"}
invalid json line
{"type":"pane","session":"test-session","window_index":0,"index":0,"active":1,"path":"/home/user","pid":12345}
"#;

        let session_output = SessionOutput {
            output: create_mock_output(mixed_output, "", true),
            target: target.clone(),
        };

        // Should fail on the invalid JSON line
        let result = Session::try_from(session_output);
        assert!(result.is_err());
    }

    #[test]
    fn test_session_target_preservation() {
        let target = Target::new("custom-session-name", Some(2), Some(1));
        let json_output = r#"{"type":"window","session":"test-session","index":0,"name":"main","active":1,"layout":"even-horizontal"}
"#;

        let session_output = SessionOutput {
            output: create_mock_output(json_output, "", true),
            target: target.clone(),
        };

        let session = Session::try_from(session_output).unwrap();

        // Session name should come from target.combined, not JSON
        assert_eq!(session.name, Some("custom-session-name:2.1".to_string()));
        assert_eq!(session.target, Some(target));
    }

    #[test]
    fn test_session_default_fields() {
        let target = Target::new("test-session", None, None);
        let json_output = r#"{"type":"window","session":"test-session","index":0,"name":"main","active":1,"layout":"even-horizontal"}
"#;

        let session_output = SessionOutput {
            output: create_mock_output(json_output, "", true),
            target: target.clone(),
        };

        let session = Session::try_from(session_output).unwrap();

        // These fields should be None/default when created from SessionOutput
        assert_eq!(session.pre, None);
        assert_eq!(session.pre_window, None);
        assert_eq!(session.root, None);
        assert_eq!(session.daemonize, None);
        assert_eq!(session.config, None);
    }
}
