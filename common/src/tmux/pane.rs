use crate::tmux::{Active, Target};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct Pane {
    pub active: Active,
    pub command: Option<String>,
    pub path: Option<PathBuf>,
    pub target: Option<Target>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum PaneRepr {
    Str(String),
    Map(PaneInner),
}

#[derive(Deserialize)]
struct PaneInner {
    #[serde(default)]
    active: Option<bool>,
    #[serde(default)]
    command: Option<String>,
    #[serde(default)]
    path: Option<PathBuf>,
}

impl<'de> Deserialize<'de> for Pane {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let repr = PaneRepr::deserialize(deserializer)?;

        match repr {
            PaneRepr::Str(cmd) => Ok(Pane {
                active: false,
                command: Some(cmd),
                path: None,
                target: None,
            }),
            PaneRepr::Map(inner) => Ok(Pane {
                active: inner.active.unwrap_or(false),
                command: inner.command,
                path: inner.path,
                target: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn deserializes_from_string() {
        let yaml = "htop";
        let pane: Pane = serde_saphyr::from_str(yaml).unwrap();
        assert_eq!(pane.active, false);
        assert_eq!(pane.command, Some("htop".to_string()));
        assert!(pane.path.is_none());
        assert!(pane.target.is_none());
    }

    #[test]
    fn deserializes_from_map_full() {
        let yaml = "\
active: true
command: ls
path: /tmp
";
        let pane: Pane = serde_saphyr::from_str(yaml).unwrap();
        assert_eq!(pane.active, true);
        assert_eq!(pane.command, Some("ls".to_string()));
        assert_eq!(pane.path.unwrap(), PathBuf::from("/tmp"));
        assert!(pane.target.is_none());
    }

    #[test]
    fn deserializes_from_map_partial() {
        let yaml = "command: ls";
        let pane: Pane = serde_saphyr::from_str(yaml).unwrap();
        assert_eq!(pane.active, false);
        assert_eq!(pane.command, Some("ls".to_string()));
        assert!(pane.path.is_none());
        assert!(pane.target.is_none());
    }

    #[test]
    fn deserializes_empty_map() {
        let yaml = "{}";
        let pane: Pane = serde_saphyr::from_str(yaml).unwrap();
        assert_eq!(pane.active, false);
        assert!(pane.command.is_none());
        assert!(pane.path.is_none());
        assert!(pane.target.is_none());
    }
}
