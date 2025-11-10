use crate::error::SnapshotError;
use common::tmux::{Pane, Target, Window};
use serde::{Deserialize, Deserializer};
use std::path::PathBuf;
use sysinfo::{Pid, Process, System};

fn bool_from_int<'de, D>(deserializer: D) -> std::result::Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let v = u8::deserialize(deserializer)?;
    Ok(v != 0)
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub(crate) enum Entity {
    #[serde(rename = "window")]
    Window {
        session: String,
        index: usize,
        name: String,
        #[serde(deserialize_with = "bool_from_int")]
        active: bool,
        layout: String,
    },
    #[serde(rename = "pane")]
    Pane {
        session: String,
        window_index: usize,
        index: usize,
        #[serde(deserialize_with = "bool_from_int")]
        active: bool,
        path: PathBuf,
        pid: usize,
    },
}

impl Entity {
    pub(crate) fn index(&self) -> usize {
        match self {
            Entity::Window { index, .. } => *index,
            Entity::Pane { index, .. } => *index,
        }
    }
}

impl TryFrom<&Entity> for Window {
    type Error = SnapshotError;

    fn try_from(entity: &Entity) -> Result<Window, SnapshotError> {
        match entity {
            Entity::Window {
                session,
                index,
                name,
                active,
                layout,
            } => {
                let target = Some(Target::new(session, Some(*index), None));

                Ok(Window {
                    active: *active,
                    command: None,
                    layout: Some(layout.clone()),
                    name: name.clone(),
                    panes: vec![],
                    path: None,
                    target,
                })
            }
            Entity::Pane { .. } => Err(SnapshotError::ToWindowFailed),
        }
    }
}

impl TryFrom<&Entity> for Pane {
    type Error = SnapshotError;

    fn try_from(entity: &Entity) -> Result<Pane, SnapshotError> {
        match entity {
            Entity::Pane {
                session,
                window_index,
                index,
                active,
                path,
                pid,
            } => {
                let target = Some(Target::new(session, Some(*window_index), Some(*index)));

                // Create a new System and refresh processes
                let mut system = System::new_all();
                system.refresh_all();

                let pid = Pid::from(*pid);
                let mut command: Option<String> = None;
                if let Some(shell_process) = system.process(pid) {
                    // Recursively descend into children
                    if let Some(inner_process) = find_foreground_process(&system, shell_process) {
                        command = Some(
                            inner_process
                                .cmd()
                                .iter()
                                .map(|s| s.to_string_lossy()) // gracefully handles invalid UTF-8
                                .collect::<Vec<_>>()
                                .join(" "),
                        )
                    }
                }

                Ok(Pane {
                    active: *active,
                    command,
                    path: Some(path.clone()),
                    target,
                })
            }
            Entity::Window { .. } => Err(SnapshotError::ToWindowFailed),
        }
    }
}

fn find_foreground_process<'a>(sys: &'a System, proc: &'a Process) -> Option<&'a Process> {
    // Collect immediate children
    let children: Vec<_> = sys
        .processes()
        .values()
        .filter(|p| p.parent() == Some(proc.pid()))
        .collect();

    // If there are no children, this shell is idle → return None
    if children.is_empty() {
        return None;
    }

    // Pick the most recently started child
    let newest = children.into_iter().max_by_key(|p| p.start_time()).unwrap();

    // Recurse into that child — maybe it spawned something else
    // If it has no children, it’s our leaf
    match find_foreground_process(sys, newest) {
        Some(grandchild) => Some(grandchild),
        None => Some(newest),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::IntoDeserializer;
    use serde_json::json;
    use std::path::PathBuf;

    #[test]
    fn test_bool_from_int() {
        // 0 == false, 1 == true, >1 == true
        let json_false = json!(0);
        let json_true1 = json!(1);
        let json_true2 = json!(42);

        let b0: bool = bool_from_int(json_false.into_deserializer()).unwrap();
        let b1: bool = bool_from_int(json_true1.into_deserializer()).unwrap();
        let b2: bool = bool_from_int(json_true2.into_deserializer()).unwrap();

        assert!(!b0);
        assert!(b1);
        assert!(b2);
    }

    #[test]
    fn test_entity_window_deserialize_and_index() {
        let s = r#"{
            "type": "window",
            "session": "mysess",
            "index": 7,
            "name": "mywin",
            "active": 1,
            "layout": "even-horizontal"
        }"#;
        let ent: Entity = serde_json::from_str(s).unwrap();
        let index_i = ent.index();
        match ent {
            Entity::Window {
                session,
                index,
                name,
                active,
                layout,
            } => {
                assert_eq!(session, "mysess");
                assert_eq!(index, 7);
                assert_eq!(name, "mywin");
                assert!(active);
                assert_eq!(layout, "even-horizontal");
                assert_eq!(index_i, 7);
            }
            _ => panic!("Expected window"),
        }
    }

    #[test]
    fn test_entity_pane_deserialize_and_index() {
        let s = r#"{
            "type": "pane",
            "session": "mysess",
            "window_index": 3,
            "index": 2,
            "active": 0,
            "path": "/tmp",
            "pid": 12345
        }"#;
        let ent: Entity = serde_json::from_str(s).unwrap();
        let index_i = ent.index();
        match ent {
            Entity::Pane {
                session,
                window_index,
                index,
                active,
                path,
                pid,
            } => {
                assert_eq!(session, "mysess");
                assert_eq!(window_index, 3);
                assert_eq!(index, 2);
                assert!(!active);
                assert_eq!(path, PathBuf::from("/tmp"));
                assert_eq!(pid, 12345);
                assert_eq!(index_i, 2);
            }
            _ => panic!("Expected pane"),
        }
    }

    #[test]
    fn test_tryfrom_entity_window_to_window_ok_and_err() {
        let ent = Entity::Window {
            session: "sess".to_string(),
            index: 1,
            name: "win".to_string(),
            active: true,
            layout: "main-vertical".to_string(),
        };
        let w = Window::try_from(&ent);
        assert!(w.is_ok());

        let ent_pane = Entity::Pane {
            session: "sess".to_string(),
            window_index: 0,
            index: 2,
            active: false,
            path: PathBuf::from("/tmp"),
            pid: 1,
        };
        let w2 = Window::try_from(&ent_pane);
        assert!(w2.is_err());
        assert_eq!(
            w2.unwrap_err().to_string(),
            "Failed to create window from snapshot"
        );
    }

    #[test]
    fn test_tryfrom_entity_pane_to_pane_ok_and_err() {
        // Only tests type check, not process scraping side-effect
        let ent = Entity::Pane {
            session: "sess".to_string(),
            window_index: 0,
            index: 2,
            active: false,
            path: PathBuf::from("/tmp"),
            pid: 1, // unlikely to find this process, but that's ok for this test
        };
        let p = Pane::try_from(&ent);
        assert!(p.is_ok());

        let ent_win = Entity::Window {
            session: "sess".to_string(),
            index: 1,
            name: "win".to_string(),
            active: true,
            layout: "main-vertical".to_string(),
        };
        let p2 = Pane::try_from(&ent_win);
        assert!(p2.is_err());
    }
}
