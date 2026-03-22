use crate::tmux::pane::Pane;
use crate::tmux::{Active, Layout, Target, is_false};
use serde::{Deserialize, Serialize, ser::SerializeMap};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Default, Clone)]
pub struct Window {
    pub active: Active,
    pub command: Option<String>,
    pub layout: Option<Layout>,
    pub name: String,
    pub panes: Vec<Pane>,
    pub path: Option<PathBuf>,
    pub target: Option<Target>,
}

// When there are no pan splits most actions are done directly to the window, but technically
// it still has a pane.
impl Window {
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Helper struct for serializing the inner fields of a Window (everything except name)
#[derive(Serialize)]
struct WindowInner<'a> {
    #[serde(skip_serializing_if = "is_false")]
    active: &'a Active,
    #[serde(skip_serializing_if = "Option::is_none")]
    command: &'a Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    layout: &'a Option<Layout>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    panes: &'a Vec<Pane>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: &'a Option<PathBuf>,
}

impl Serialize for Window {
    /// Custom serializer that outputs the map format: `{name: {inner_fields...}}`
    ///
    /// This produces YAML like:
    /// ```yaml
    /// editor:
    ///   layout: even-horizontal
    ///   panes: [htop, ranger]
    ///   active: true
    /// ```
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let inner = WindowInner {
            active: &self.active,
            command: &self.command,
            layout: &self.layout,
            panes: &self.panes,
            path: &self.path,
        };

        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.name, &inner)?;
        map.end()
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum WindowRepr {
    // windows: ['vim', 'cargo']
    Str(String),

    // windows: [1, 2, 3]
    Num(i64),

    // windows:
    //   - name: editor
    //     layout: main-vertical
    //     panes: [...]
    Direct(DirectWindow),

    // windows:
    //   - editor:
    //       layout: main-vertical
    //       panes: [...]
    //   - cargo: ''
    Map(BTreeMap<String, InnerOrString>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum InnerOrString {
    Str(String),
    Inner(Inner),
}

#[derive(Debug, Default, Deserialize)]
struct Inner {
    pub layout: Option<Layout>,
    pub panes: Option<Vec<Pane>>,
    pub active: Option<Active>,
    pub path: Option<PathBuf>,
    pub command: Option<String>,
}

/// Direct window format with name as a field (legacy format for backward compatibility)
#[derive(Debug, Deserialize)]
struct DirectWindow {
    pub name: String,
    #[serde(default)]
    pub layout: Option<Layout>,
    #[serde(default)]
    pub panes: Option<Vec<Pane>>,
    #[serde(default)]
    pub active: Option<Active>,
    #[serde(default)]
    pub path: Option<PathBuf>,
    #[serde(default)]
    pub command: Option<String>,
}

impl<'de> Deserialize<'de> for Window {
    /// Custom deserializer for the `Window` struct, supporting multiple YAML representations.
    ///
    /// This implementation allows a `Window` to be deserialized from:
    /// - A string (used as both the window name and the command)
    /// - An integer (used as the window name, with no command)
    /// - A single-key map, where the key is the window name and the value is either:
    ///   - A string (used as the command)
    ///   - An object with fields `layout`, `panes`, `active`, `path`, and/or `command`
    ///
    /// Examples of supported YAML representations:
    /// ```yaml
    /// # As a string:
    /// windows: [vim]
    ///
    /// # As a number:
    /// windows: [1]
    ///
    /// # As a map with a command string:
    /// windows:
    ///   - edit: vim
    ///
    /// # As a map with a detailed object:
    /// windows:
    ///   - term:
    ///       layout: even-horizontal
    ///       panes: [htop, ranger]
    ///       active: true
    ///       command: mycmd
    ///       path: /tmp
    /// ```
    ///
    /// - If the map contains more than one key or an empty key, an error is returned.
    /// - If parsing as a string or integer, defaults are filled for missing fields.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let repr = WindowRepr::deserialize(deserializer)?;
        match repr {
            WindowRepr::Str(name) => Ok(Window {
                name: name.clone(),
                active: false,
                layout: None,
                path: None,
                command: Some(name),
                panes: vec![],
                target: None,
            }),
            WindowRepr::Num(n) => Ok(Window {
                name: n.to_string(),
                active: false,
                layout: None,
                path: None,
                command: None,
                panes: vec![],
                target: None,
            }),
            WindowRepr::Map(map) => {
                if map.len() != 1 {
                    return Err(serde::de::Error::custom(
                        "each windows entry must be a single-key map",
                    ));
                }

                let (name, ios) = map.into_iter().next().unwrap();

                // serde_saphyr replaces null with ~ which I guess is a yaml thing.
                if name.is_empty() || name == "~" {
                    return Err(serde::de::Error::custom("window name cannot be empty"));
                }

                match ios {
                    InnerOrString::Str(cmd) => Ok(Window {
                        active: false,
                        command: Some(cmd.clone()),
                        layout: None,
                        name,
                        panes: vec![],
                        path: None,
                        target: None,
                    }),
                    InnerOrString::Inner(inner) => Ok(Window {
                        name,
                        active: inner.active.unwrap_or(false),
                        command: inner.command,
                        layout: inner.layout,
                        panes: inner.panes.unwrap_or_default(),
                        path: inner.path,
                        target: None,
                    }),
                }
            }
            WindowRepr::Direct(direct) => Ok(Window {
                name: direct.name,
                active: direct.active.unwrap_or(false),
                command: direct.command,
                layout: direct.layout,
                panes: direct.panes.unwrap_or_default(),
                path: direct.path,
                target: None,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_saphyr;

    #[test]
    fn deserializes_from_string() {
        let yaml = "vim";
        let window: Window = serde_saphyr::from_str(yaml).unwrap();
        assert_eq!(window.name, "vim");
        assert_eq!(window.command, Some("vim".to_string()));
        assert_eq!(window.panes.len(), 0);
        assert!(!window.active);
    }

    #[test]
    fn deserializes_from_integer() {
        let yaml = "42";
        let window: Window = serde_saphyr::from_str(yaml).unwrap();
        assert_eq!(window.name, "42");
        assert!(window.command.is_none());
        assert_eq!(window.panes.len(), 0);
        assert!(!window.active);
    }

    #[test]
    fn deserializes_from_map_with_string() {
        let yaml = "edit: vim";
        let window: Window = serde_saphyr::from_str(yaml).unwrap();
        assert_eq!(window.name, "edit");
        assert_eq!(window.command, Some("vim".to_string()));
        assert_eq!(window.panes.len(), 0);
        assert!(!window.active);
    }

    #[test]
    fn deserializes_from_map_with_inner() {
        let yaml = "\
term:
  layout: even-horizontal
  panes:
    - htop
    - ranger
  active: true
  command: mycmd
  path: /tmp
";
        let window: Window = serde_saphyr::from_str(yaml).unwrap();
        assert_eq!(window.name, "term");
        assert_eq!(
            window.layout.as_ref().unwrap().to_string(),
            "even-horizontal"
        );
        assert_eq!(window.panes.len(), 2);
        assert_eq!(window.panes[0].command.as_ref().unwrap(), "htop");
        assert_eq!(window.panes[1].command.as_ref().unwrap(), "ranger");
        assert_eq!(window.active, true);
        assert_eq!(window.command.as_ref().unwrap(), "mycmd");
        assert_eq!(window.path.as_ref().unwrap().to_str().unwrap(), "/tmp");
    }

    #[test]
    fn errors_on_map_with_multiple_keys() {
        let yaml = "foo: bar\nbaz: qux";
        let error = serde_saphyr::from_str::<Window>(yaml);
        assert!(error.is_err());
    }

    #[test]
    fn errors_on_map_with_empty_key() {
        let yaml = ": bar";
        let error = serde_saphyr::from_str::<Window>(yaml);
        assert!(error.is_err());
    }

    #[test]
    fn serializes_to_map_format() {
        let window = Window {
            name: "editor".to_string(),
            active: true,
            layout: Some("even-horizontal".into()),
            panes: vec![],
            command: Some("vim".to_string()),
            path: Some(PathBuf::from("/tmp")),
            target: None,
        };

        let yaml = serde_saphyr::to_string(&window).unwrap();
        // Should serialize as: editor: {active: true, command: vim, layout: even-horizontal, path: /tmp}
        assert!(yaml.contains("editor:"));
        assert!(!yaml.contains("name:"));
    }

    #[test]
    fn roundtrip_serialization() {
        let window = Window {
            name: "term".to_string(),
            active: true,
            layout: Some("even-horizontal".into()),
            panes: vec![],
            command: Some("mycmd".to_string()),
            path: Some(PathBuf::from("/tmp")),
            target: None,
        };

        let yaml = serde_saphyr::to_string(&window).unwrap();
        let parsed: Window = serde_saphyr::from_str(&yaml).unwrap();

        assert_eq!(parsed.name, window.name);
        assert_eq!(parsed.active, window.active);
        assert_eq!(parsed.layout, window.layout);
        assert_eq!(parsed.command, window.command);
        assert_eq!(parsed.path, window.path);
    }

    #[test]
    fn deserializes_legacy_direct_format() {
        // Legacy format with name as a field (for backward compatibility)
        let yaml = "\
name: editor
layout: even-horizontal
panes:
  - htop
  - ranger
active: true
command: mycmd
path: /tmp
";
        let window: Window = serde_saphyr::from_str(yaml).unwrap();
        assert_eq!(window.name, "editor");
        assert_eq!(
            window.layout.as_ref().unwrap().to_string(),
            "even-horizontal"
        );
        assert_eq!(window.panes.len(), 2);
        assert!(window.active);
        assert_eq!(window.command.as_ref().unwrap(), "mycmd");
        assert_eq!(window.path.as_ref().unwrap().to_str().unwrap(), "/tmp");
    }
}
