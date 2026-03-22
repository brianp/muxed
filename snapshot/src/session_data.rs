use crate::entity::Entity;
use crate::error::SnapshotError;
use common::DEBUG;
use common::tmux::{Pane, Session, Target, Window};
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use std::process::Output;

type Result<T> = std::result::Result<T, SnapshotError>;

/// Derive the most common path among all panes to use as the session root.
/// Only returns a root if a path appears more than once (i.e., is actually common).
fn derive_root_path(windows: &[&mut Window]) -> Option<PathBuf> {
    let mut path_counts: HashMap<PathBuf, usize> = HashMap::new();

    for window in windows {
        for pane in &window.panes {
            if let Some(path) = &pane.path {
                *path_counts.entry(path.clone()).or_insert(0) += 1;
            }
        }
    }

    // Only return a path if it appears more than once (is actually common)
    path_counts
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .max_by_key(|(_, count)| *count)
        .map(|(path, _)| path)
}

/// Derive the most common path among panes in a window.
/// Returns the path only if it's used by the majority of panes.
fn derive_window_path(panes: &[Pane]) -> Option<PathBuf> {
    if panes.is_empty() {
        return None;
    }

    let mut path_counts: HashMap<PathBuf, usize> = HashMap::new();

    for pane in panes {
        if let Some(path) = &pane.path {
            *path_counts.entry(path.clone()).or_insert(0) += 1;
        }
    }

    // Return the most common path if it's the majority
    path_counts
        .iter()
        .max_by_key(|(_, count)| *count)
        .filter(|(_, count)| **count * 2 > panes.len())
        .map(|(path, _)| path.clone())
}

/// Clear redundant paths that match the root or window path.
fn clear_redundant_paths(windows: &mut BTreeMap<usize, Window>, root: &Option<PathBuf>) {
    for window in windows.values_mut() {
        // If window path matches root, clear it
        if let (Some(wp), Some(r)) = (&window.path, root)
            && wp == r
        {
            window.path = None;
        }

        for pane in &mut window.panes {
            // If pane path matches window path or root, clear it
            let clear = match (&pane.path, &window.path, root) {
                (Some(p), Some(wp), _) if p == wp => true,
                (Some(p), None, Some(r)) if p == r => true,
                _ => false,
            };
            if clear {
                pane.path = None;
            }
        }
    }
}

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

        // Derive window paths from panes (most common path in each window)
        for window in windows.values_mut() {
            window.path = derive_window_path(&window.panes);
        }

        // Derive root path from all panes (most common path overall)
        let window_refs: Vec<&mut Window> = windows.values_mut().collect();
        let root = derive_root_path(&window_refs);

        // Clear redundant paths that match root or window path
        clear_redundant_paths(&mut windows, &root);

        Ok(Session {
            name: Some(session.target.combined.clone()),
            pre: None,
            pre_window: None,
            root,
            windows: windows.into_values().collect(),
            target: Some(session.target),
            daemonize: None,
            config: None,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::os::unix::process::ExitStatusExt;
    use std::process::Output;

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

    #[test]
    fn test_derives_root_from_common_pane_paths() {
        let target = Target::new("test-session", None, None);
        // All 3 panes have /tmp as their path
        let json_output = r#"{"type":"window","session":"test-session","index":0,"name":"main","active":1,"layout":"even-horizontal"}
{"type":"pane","session":"test-session","window_index":0,"index":0,"active":1,"path":"/tmp","pid":12345}
{"type":"pane","session":"test-session","window_index":0,"index":1,"active":0,"path":"/tmp","pid":12346}
{"type":"pane","session":"test-session","window_index":0,"index":2,"active":0,"path":"/tmp","pid":12347}
"#;

        let session_output = SessionOutput {
            output: create_mock_output(json_output, "", true),
            target: target.clone(),
        };

        let session = Session::try_from(session_output).unwrap();

        // Root should be /tmp since all panes share it
        assert_eq!(session.root, Some(std::path::PathBuf::from("/tmp")));

        // Window path should also be /tmp (majority of panes)
        let window = &session.windows[0];
        assert_eq!(window.path, None); // Cleared because it matches root

        // Pane paths should be cleared since they match root
        for pane in &window.panes {
            assert_eq!(pane.path, None);
        }
    }

    #[test]
    fn test_derives_root_from_majority_pane_paths() {
        let target = Target::new("test-session", None, None);
        // 2 panes have /tmp, 1 has /home
        let json_output = r#"{"type":"window","session":"test-session","index":0,"name":"main","active":1,"layout":"even-horizontal"}
{"type":"pane","session":"test-session","window_index":0,"index":0,"active":1,"path":"/tmp","pid":12345}
{"type":"pane","session":"test-session","window_index":0,"index":1,"active":0,"path":"/tmp","pid":12346}
{"type":"pane","session":"test-session","window_index":0,"index":2,"active":0,"path":"/home","pid":12347}
"#;

        let session_output = SessionOutput {
            output: create_mock_output(json_output, "", true),
            target: target.clone(),
        };

        let session = Session::try_from(session_output).unwrap();

        // Root should be /tmp since it appears in 2 panes (more than once)
        assert_eq!(session.root, Some(std::path::PathBuf::from("/tmp")));

        // Window should have /tmp as path (2 out of 3 is majority)
        let window = &session.windows[0];
        assert_eq!(window.path, None); // Cleared because it matches root

        // First two panes should have path cleared (matches root)
        assert_eq!(window.panes[0].path, None);
        assert_eq!(window.panes[1].path, None);
        // Third pane keeps its unique path
        assert_eq!(
            window.panes[2].path,
            Some(std::path::PathBuf::from("/home"))
        );
    }

    #[test]
    fn test_no_root_when_all_paths_unique() {
        let target = Target::new("test-session", None, None);
        // Each pane has a unique path
        let json_output = r#"{"type":"window","session":"test-session","index":0,"name":"main","active":1,"layout":"even-horizontal"}
{"type":"pane","session":"test-session","window_index":0,"index":0,"active":1,"path":"/home/user","pid":12345}
{"type":"pane","session":"test-session","window_index":0,"index":1,"active":0,"path":"/tmp","pid":12346}
{"type":"pane","session":"test-session","window_index":0,"index":2,"active":0,"path":"/var","pid":12347}
"#;

        let session_output = SessionOutput {
            output: create_mock_output(json_output, "", true),
            target: target.clone(),
        };

        let session = Session::try_from(session_output).unwrap();

        // No root since no path appears more than once
        assert_eq!(session.root, None);

        // Window path should be None (no majority)
        let window = &session.windows[0];
        assert_eq!(window.path, None);

        // All panes should retain their original paths
        assert_eq!(
            window.panes[0].path,
            Some(std::path::PathBuf::from("/home/user"))
        );
        assert_eq!(window.panes[1].path, Some(std::path::PathBuf::from("/tmp")));
        assert_eq!(window.panes[2].path, Some(std::path::PathBuf::from("/var")));
    }

    #[test]
    fn test_window_path_derived_from_pane_majority() {
        let target = Target::new("test-session", None, None);
        // Window 0: 2/3 panes have /project, Window 1: all different
        let json_output = r#"{"type":"window","session":"test-session","index":0,"name":"work","active":1,"layout":"even-horizontal"}
{"type":"window","session":"test-session","index":1,"name":"misc","active":0,"layout":"even-horizontal"}
{"type":"pane","session":"test-session","window_index":0,"index":0,"active":1,"path":"/project","pid":12345}
{"type":"pane","session":"test-session","window_index":0,"index":1,"active":0,"path":"/project","pid":12346}
{"type":"pane","session":"test-session","window_index":0,"index":2,"active":0,"path":"/tmp","pid":12347}
{"type":"pane","session":"test-session","window_index":1,"index":0,"active":0,"path":"/home","pid":12348}
{"type":"pane","session":"test-session","window_index":1,"index":1,"active":0,"path":"/var","pid":12349}
"#;

        let session_output = SessionOutput {
            output: create_mock_output(json_output, "", true),
            target: target.clone(),
        };

        let session = Session::try_from(session_output).unwrap();

        // Root should be /project (appears twice, which is more than once)
        assert_eq!(session.root, Some(std::path::PathBuf::from("/project")));

        // First window should have path cleared (matches root)
        let window0 = &session.windows[0];
        assert_eq!(window0.path, None);

        // Second window should have None (no majority)
        let window1 = &session.windows[1];
        assert_eq!(window1.path, None);
    }
}
