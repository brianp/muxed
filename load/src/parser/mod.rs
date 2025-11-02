//! The YAML parser. Here is where we convert the yaml in to commands to be
//! processed later.

use crate::command::*;
use crate::project;
use crate::tmux::config::Config;
use crate::tmux::target::*;
use common::project_paths::homedir;
use error::ParseError;
use std::path::PathBuf;
use std::rc::Rc;
use yaml_rust::Yaml;

pub mod error;

type Result<T> = std::result::Result<T, ParseError>;

/// Here was pass in the parsed yaml and project name. The purpose of this call
/// loop is to build the stack of commands that are run to setup a users tmux
/// session.
///
/// `yaml_string`: The parsed yaml from the config file.
/// `project_name`: The name of the project.
pub fn call<'a>(
    yaml_string: &'a [Yaml],
    project_name: &'a str,
    daemonize: bool,
    tmux_config: &Config,
) -> Result<Vec<Commands>> {
    let mut commands: Vec<Commands> = vec![];
    let project_name = Rc::new(project_name);

    // There should only be one doc but it's a vec so take the first.
    let doc = &yaml_string.first().ok_or(ParseError::NoDocFound)?;

    let root = expand_path(&doc["root"])?;
    let pre_window = pre_matcher(&doc["pre_window"]);

    // A closure used to capture the current local root and pre Options.
    // This way we can call the closure to create common SendKeys command
    // like changing the directory or executing a system command from the
    // `pre_window` option.
    let common_commands = |target: Target| -> Vec<Commands> {
        let mut commands2 = vec![];

        // SendKeys for the Pre option
        if let Some(p) = pre_window.clone() {
            for v in &p {
                if let Some(ref r) = *v {
                    commands2.push(SendKeys::new(target.clone(), r.clone()).into());
                };
            }
        };

        commands2
    };

    let windows = doc["windows"].as_vec().ok_or(ParseError::NoWindows)?;

    for window in windows.iter() {
        match *window {
            Yaml::Hash(ref h) => {
                for (k, v) in h {
                    let key = k.as_str().ok_or(ParseError::WindowNameRequired)?;
                    if v.as_hash().is_some() {
                        let path = match expand_path(&v["path"]) {
                            Ok(x) => x,
                            Err(_) => root.clone(),
                        };

                        commands.push(
                            Window::new(&project_name, Rc::new(key.to_string()), path.clone())
                                .into(),
                        );

                        let target = WindowTarget::new(Rc::clone(&project_name), key);
                        commands.append(&mut pane_matcher(
                            v,
                            &target,
                            common_commands,
                            tmux_config,
                            path.clone(),
                        )?);
                    } else {
                        commands.push(
                            Window::new(&project_name, Rc::new(key.to_string()), root.clone())
                                .into(),
                        );

                        let target = WindowTarget::new(
                            Rc::clone(&project_name),
                            k.as_str().ok_or(ParseError::WindowTargetRequired)?,
                        );
                        commands.append(&mut common_commands(Target::WindowTarget(target.clone())));

                        // SendKeys for the exec command if not empty
                        if let Some(exec) = v.as_str()
                            && !exec.is_empty()
                        {
                            commands.push(
                                SendKeys::new(
                                    Target::WindowTarget(target.clone()),
                                    exec.to_string(),
                                )
                                .into(),
                            );
                        };
                    }
                }
            }
            Yaml::String(ref s) => {
                commands
                    .push(Window::new(&project_name, Rc::new(s.to_string()), root.clone()).into());

                let target = WindowTarget::new(Rc::clone(&project_name), s);
                commands.append(&mut common_commands(Target::WindowTarget(target)));
            }
            Yaml::Integer(ref s) => {
                commands.push(
                    Window::new(&project_name, Rc::new(format!("{}", s)), root.clone()).into(),
                );

                let target = WindowTarget::new(Rc::clone(&project_name), &s.to_string());
                commands.append(&mut common_commands(Target::WindowTarget(target)));
            }
            _ => return Err(ParseError::FormatNotRecognized),
        };
    }

    let (first, commands1) = commands.split_first().ok_or(ParseError::BadCommandSplit)?;
    let mut remains = commands1.to_vec();

    if let Commands::Window(w) = &first {
        remains.insert(
            0,
            Session::new(&project_name, Rc::clone(&w.name), root.clone()).into(),
        );

        if let Some(path) = &w.path {
            remains.insert(
                1,
                SendKeys::new(
                    Target::WindowTarget(WindowTarget::new(Rc::clone(&project_name), &w.name)),
                    format!("cd {}", path.display()),
                )
                .into(),
            );
        }

        remains
            .push(SelectWindow::new(WindowTarget::new(Rc::clone(&project_name), &w.name)).into());
        remains.push(
            SelectPane::new(PaneTarget::new(
                &project_name,
                &w.name,
                tmux_config.base_index,
            ))
            .into(),
        );
    };

    // FIXME: Due to inserting the Pre commands into the 0 position in the stack,
    // if pre is defined as an array, it is executed in reverse order.
    let pre = pre_matcher(&doc["pre"]);
    if let Some(ref p) = pre {
        for v in p.iter() {
            if let Some(ref r) = *v {
                remains.insert(0, Pre::new(r.clone()).into());
            };
        }
    };

    if !daemonize {
        remains.push(project::open(&project_name));
    };

    Ok(remains)
}

/// Pane matcher is for breaking apart the panes. Splitting windows when needed
/// and executing commands as needed.
fn pane_matcher<T>(
    window: &Yaml,
    target: &WindowTarget,
    common_commands: T,
    tmux_config: &Config,
    inherited_path: Option<Rc<PathBuf>>,
) -> Result<Vec<Commands>>
where
    T: Fn(Target) -> Vec<Commands>,
{
    let mut commands = vec![];
    let panes = window["panes"]
        .as_vec()
        .ok_or(ParseError::PanesConversion)?;

    let path = expand_path(&window["path"]).unwrap_or(inherited_path);

    for (i, pane) in panes.iter().enumerate() {
        let pt = PaneTarget::new(
            &target.session,
            &target.window,
            i + tmux_config.pane_base_index,
        );
        // For every pane, we need one less split.
        // ex. An existing window to become 2 panes, needs 1 split.
        if i < (panes.len() - 1) {
            commands.push(Split::new(pt.clone(), path.clone()).into());
        };

        // Call the common_commands closure to execute `cd` and `pre_window` options in
        // pane splits.
        commands.append(&mut common_commands(Target::PaneTarget(pt.clone())));

        // Execute given commands in each new pane after all splits are
        // complete.
        if let Some(p) = pane.as_str()
            && !p.is_empty()
        {
            commands.push(SendKeys::new(Target::PaneTarget(pt.clone()), p.to_string()).into());
        };
    }

    // After all panes are split select the layout for the window
    if let Some(layout) = window["layout"].as_str() {
        commands.push(Layout::new(target.clone(), layout.to_string()).into());
    }

    Ok(commands)
}

fn pre_matcher(node: &Yaml) -> Option<Vec<Option<String>>> {
    match *node {
        // See if pre contains an array or a string. If it's an array we
        // need to check the values of it again to verify they are strings.
        Yaml::String(ref x) => Some(vec![Some(x.to_string())]),
        Yaml::Array(ref x) => Some(
            x.iter()
                .map(|y| match *y {
                    Yaml::String(ref z) => Some(z.to_string()),
                    _ => None,
                })
                .collect(),
        ),
        _ => None,
    }
}

fn expand_path(node: &Yaml) -> Result<Option<Rc<PathBuf>>> {
    let s = match node.as_str() {
        Some(s) => s,
        None => return Ok(None),
    };

    let expanded = if let Some(stripped) = s.strip_prefix("~/") {
        match homedir() {
            Some(home) => Rc::new(home.join(stripped)),
            None => return Err(ParseError::FormatNotRecognized),
        }
    } else {
        Rc::new(PathBuf::from(s))
    };
    Ok(Some(expanded))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;
    use yaml_rust::YamlLoader;

    #[test]
    pub fn expect_1_session() {
        let s = "---
    windows: ['cargo', 'vim', 'git']
    ";
        let yaml = YamlLoader::load_from_str(s).unwrap();
        let remains: Vec<Commands> = call(
            &yaml,
            "muxed",
            false,
            &Config {
                base_index: 0,
                pane_base_index: 0,
            },
        )
        .unwrap()
        .into_iter()
        .filter(|x| match x {
            Commands::Session(_) => true,
            _ => false,
        })
        .collect();

        assert_eq!(remains.len(), 1)
    }

    #[test]
    pub fn expect_2_windows_from_array() {
        let s = "---
    windows: ['cargo', 'vim', 'git']
    ";
        let yaml = YamlLoader::load_from_str(s).unwrap();
        let remains: Vec<Commands> = call(
            &yaml,
            "muxed",
            false,
            &Config {
                base_index: 0,
                pane_base_index: 0,
            },
        )
        .unwrap()
        .into_iter()
        .filter(|x| match x {
            Commands::Window(_) => true,
            _ => false,
        })
        .collect();

        assert_eq!(remains.len(), 2)
    }

    #[test]
    pub fn expect_1_attach() {
        let s = "---
    windows: ['cargo', 'vim', 'git']
    ";
        let yaml = YamlLoader::load_from_str(s).unwrap();
        let remains: Vec<Commands> = call(
            &yaml,
            "muxed",
            false,
            &Config {
                base_index: 0,
                pane_base_index: 0,
            },
        )
        .unwrap()
        .into_iter()
        .filter(|x| match x {
            Commands::Attach(_) => true,
            _ => false,
        })
        .collect();

        assert_eq!(remains.len(), 1)
    }

    #[test]
    pub fn expect_2_windows_with_mixed_type_names() {
        let s = "---
    windows: [1, 'vim', 3]
    ";
        let yaml = YamlLoader::load_from_str(s).unwrap();
        let remains: Vec<Commands> = call(
            &yaml,
            "muxed",
            false,
            &Config {
                base_index: 0,
                pane_base_index: 0,
            },
        )
        .unwrap()
        .into_iter()
        .filter(|x| match x {
            Commands::Window(_) => true,
            _ => false,
        })
        .collect();
        assert_eq!(remains.len(), 2)
    }

    #[test]
    pub fn expect_2_windows_from_list() {
        let s = "---
    windows:
      - cargo: ''
      - vim: ''
      - git: ''
    ";
        let yaml = YamlLoader::load_from_str(s).unwrap();
        let remains: Vec<Commands> = call(
            &yaml,
            "muxed",
            false,
            &Config {
                base_index: 0,
                pane_base_index: 0,
            },
        )
        .unwrap()
        .into_iter()
        .filter(|x| match x {
            Commands::Window(_) => true,
            _ => false,
        })
        .collect();
        assert_eq!(remains.len(), 2)
    }

    #[test]
    pub fn expect_ok_with_empty_syscommands() {
        let s = "---
    windows:
      - editor:
    ";
        let yaml = YamlLoader::load_from_str(s).unwrap();
        let result = call(
            &yaml,
            "muxed",
            false,
            &Config {
                base_index: 0,
                pane_base_index: 0,
            },
        );
        assert!(result.is_ok())
    }

    #[test]
    pub fn expect_no_send_keys_commands() {
        let s = "---
    windows:
      - editor:
    ";

        let yaml = YamlLoader::load_from_str(s).unwrap();
        let remains: Vec<Commands> = call(
            &yaml,
            "muxed",
            false,
            &Config {
                base_index: 0,
                pane_base_index: 0,
            },
        )
        .unwrap()
        .into_iter()
        .filter(|x| match x {
            Commands::SendKeys(_) => true,
            _ => false,
        })
        .collect();

        assert_eq!(remains.len(), 0)
    }

    #[test]
    pub fn expect_err_with_nameless_window() {
        let s = "---
    windows:
      - : ls
    ";
        let yaml = YamlLoader::load_from_str(s).unwrap();
        let result = call(
            &yaml,
            "muxed",
            false,
            &Config {
                base_index: 0,
                pane_base_index: 0,
            },
        );
        assert!(result.is_err())
    }

    #[test]
    pub fn expect_ok_with_empty_panes_syscommands() {
        let s = "---
    windows:
      - cargo:
          layout: 'main-vertical'
          panes:
            -
    ";
        let yaml = YamlLoader::load_from_str(s).unwrap();
        let result = call(
            &yaml,
            "muxed",
            false,
            &Config {
                base_index: 0,
                pane_base_index: 0,
            },
        );
        assert!(result.is_ok())
    }

    #[test]
    pub fn expect_no_send_keys_with_empty_panes_syscommands() {
        let s = "---
    windows:
      - editor:
          panes:
            -
    ";

        let yaml = YamlLoader::load_from_str(s).unwrap();
        let remains: Vec<Commands> = call(
            &yaml,
            "muxed",
            false,
            &Config {
                base_index: 0,
                pane_base_index: 0,
            },
        )
        .unwrap()
        .into_iter()
        .filter(|x| match x {
            Commands::SendKeys(_) => true,
            _ => false,
        })
        .collect();

        assert_eq!(remains.len(), 0)
    }

    #[test]
    pub fn expect_1_split_window() {
        let s = "---
    windows:
      - editor:
          layout: 'main-vertical'
          panes: ['vim', 'guard']
    ";

        let yaml = YamlLoader::load_from_str(s).unwrap();
        let remains: Vec<Commands> = call(
            &yaml,
            "muxed",
            false,
            &Config {
                base_index: 0,
                pane_base_index: 0,
            },
        )
        .unwrap()
        .into_iter()
        .filter(|x| match x {
            Commands::Split(_) => true,
            _ => false,
        })
        .collect();

        assert_eq!(remains.len(), 1)
    }

    #[test]
    pub fn expect_1_layout() {
        let s = "---
    windows:
      - editor:
          layout: 'main-vertical'
          panes: ['vim', 'guard']
    ";

        let yaml = YamlLoader::load_from_str(s).unwrap();
        let remains: Vec<Commands> = call(
            &yaml,
            "muxed",
            false,
            &Config {
                base_index: 0,
                pane_base_index: 0,
            },
        )
        .unwrap()
        .into_iter()
        .filter(|x| match x {
            Commands::Layout(_) => true,
            _ => false,
        })
        .collect();

        assert_eq!(remains.len(), 1)
    }

    #[test]
    pub fn expect_1_session_with_panes_array() {
        let s = "---
    windows:
      - editor:
          layout: 'main-vertical'
          panes: ['vim', 'guard']
    ";

        let yaml = YamlLoader::load_from_str(s).unwrap();
        let remains: Vec<Commands> = call(
            &yaml,
            "muxed",
            false,
            &Config {
                base_index: 0,
                pane_base_index: 0,
            },
        )
        .unwrap()
        .into_iter()
        .filter(|x| match x {
            Commands::Session(_) => true,
            _ => false,
        })
        .collect();

        assert_eq!(remains.len(), 1)
    }

    #[test]
    pub fn expect_no_layout() {
        let s = "---
    windows:
      - editor:
          panes: ['vim', 'guard']
    ";

        let yaml = YamlLoader::load_from_str(s).unwrap();
        let remains: Vec<Commands> = call(
            &yaml,
            "muxed",
            false,
            &Config {
                base_index: 0,
                pane_base_index: 0,
            },
        )
        .unwrap()
        .into_iter()
        .filter(|x| match x {
            Commands::Layout(_) => true,
            _ => false,
        })
        .collect();

        assert_eq!(remains.len(), 0)
    }

    #[test]
    pub fn expect_three_send_keys_commands_from_pre_window() {
        // pre gets run on all 2 panes and 1 window for a total of 3
        let s = "---
    pre_window: 'ls'
    windows:
      - editor:
          panes:
            -
            -
      - logs:
    ";
        let yaml = YamlLoader::load_from_str(s).unwrap();
        let remains: Vec<Commands> = call(
            &yaml,
            "muxed",
            false,
            &Config {
                base_index: 0,
                pane_base_index: 0,
            },
        )
        .unwrap()
        .into_iter()
        .filter(|x| match x {
            Commands::SendKeys(_) => true,
            _ => false,
        })
        .collect();

        assert_eq!(remains.len(), 3)
    }

    #[test]
    pub fn expect_two_send_keys_commands_from_pre_window() {
        let s = "---
    pre_window:
     - 'ls'
     - 'ls'
    windows:
      - editor:
    ";
        let yaml = YamlLoader::load_from_str(s).unwrap();
        let remains: Vec<Commands> = call(
            &yaml,
            "muxed",
            false,
            &Config {
                base_index: 0,
                pane_base_index: 0,
            },
        )
        .unwrap()
        .into_iter()
        .filter(|x| match x {
            Commands::SendKeys(_) => true,
            _ => false,
        })
        .collect();

        assert_eq!(remains.len(), 2)
    }

    #[test]
    pub fn expect_no_send_keys_with_blank_panes() {
        let s = "---
    windows:
      - editor:
          panes: ['','','']
    ";

        let yaml = YamlLoader::load_from_str(s).unwrap();
        let remains: Vec<Commands> = call(
            &yaml,
            "muxed",
            false,
            &Config {
                base_index: 0,
                pane_base_index: 0,
            },
        )
        .unwrap()
        .into_iter()
        .filter(|x| match x {
            Commands::SendKeys(_) => true,
            _ => false,
        })
        .collect();

        assert_eq!(remains.len(), 0)
    }

    #[test]
    pub fn expect_no_send_keys_with_blank_window() {
        let s = "---
    windows:
      - editor: ''
    ";

        let yaml = YamlLoader::load_from_str(s).unwrap();
        let remains: Vec<Commands> = call(
            &yaml,
            "muxed",
            false,
            &Config {
                base_index: 0,
                pane_base_index: 0,
            },
        )
        .unwrap()
        .into_iter()
        .filter(|x| match x {
            Commands::SendKeys(_) => true,
            _ => false,
        })
        .collect();

        assert_eq!(remains.len(), 0)
    }

    #[test]
    pub fn expect_full_directory_name() {
        let s = "---
    root: ~/JustPlainSimple Technologies Inc./financials/ledgers
    windows:
      - dir: ''
    ";

        let yaml = YamlLoader::load_from_str(s).unwrap();
        let remains: Commands = call(
            &yaml,
            "financials",
            false,
            &Config {
                base_index: 0,
                pane_base_index: 0,
            },
        )
        .unwrap()
        .into_iter()
        .find(|x| match x {
            Commands::Session(_) => true,
            _ => false,
        })
        .unwrap();

        let root = match remains {
            Commands::Session(ref k) => k,
            _ => panic!("nope"),
        };

        let home = dirs::home_dir().unwrap();
        let path = PathBuf::from("JustPlainSimple Technologies Inc./financials/ledgers");

        assert_eq!(root.root_path, Some(Rc::new(home.join(path))))
    }

    #[test]
    pub fn expect_1_select_window() {
        let s = "---
    windows: ['cargo', 'vim', 'git']
    ";
        let yaml = YamlLoader::load_from_str(s).unwrap();
        let remains: Vec<Commands> = call(
            &yaml,
            "muxed",
            false,
            &Config {
                base_index: 0,
                pane_base_index: 0,
            },
        )
        .unwrap()
        .into_iter()
        .filter(|x| match x {
            Commands::SelectWindow(_) => true,
            _ => false,
        })
        .collect();

        assert_eq!(remains.len(), 1)
    }

    #[test]
    pub fn expect_1_select_pane() {
        let s = "---
    windows: ['cargo', 'vim', 'git']
    ";
        let yaml = YamlLoader::load_from_str(s).unwrap();
        let remains: Vec<Commands> = call(
            &yaml,
            "muxed",
            false,
            &Config {
                base_index: 0,
                pane_base_index: 0,
            },
        )
        .unwrap()
        .into_iter()
        .filter(|x| match x {
            Commands::SelectPane(_) => true,
            _ => false,
        })
        .collect();

        assert_eq!(remains.len(), 1)
    }

    #[test]
    pub fn expect_vec_of_option_string() {
        let s = "---
    pre: ls -alh
    ";
        let yaml = YamlLoader::load_from_str(s).unwrap();
        let pre = pre_matcher(&yaml[0]["pre"]);
        assert_eq!(pre.unwrap(), vec!(Some("ls -alh".to_string())))
    }

    #[test]
    pub fn expect_vec_of_option_strings() {
        let s = "---
    pre:
      - ls -alh
      - tail -f
    ";
        let yaml = YamlLoader::load_from_str(s).unwrap();
        let pre = pre_matcher(&yaml[0]["pre"]);
        assert_eq!(
            pre.unwrap(),
            vec!(Some("ls -alh".to_string()), Some("tail -f".to_string()))
        )
    }

    #[test]
    pub fn expect_some_from_pre_matcher() {
        let s = "---
    pre: ls -alh
    ";
        let yaml = YamlLoader::load_from_str(s).unwrap();
        let pre = pre_matcher(&yaml[0]["pre"]);
        assert!(pre.is_some())
    }

    #[test]
    pub fn expect_none_from_pre_matcher() {
        let s = "---
    pre:
    ";
        let yaml = YamlLoader::load_from_str(s).unwrap();
        let pre = pre_matcher(&yaml[0]["pre"]);
        assert!(pre.is_none())
    }

    #[test]
    fn expand_path_none_if_input_not_str() {
        // Yaml::Integer is not a string type
        let yaml = Yaml::Integer(123);
        let r = expand_path(&yaml).unwrap();
        assert_eq!(r, None);
    }

    #[test]
    fn expand_path_plain_abs_path() {
        let yaml = Yaml::String("/tmp/foo".into());
        let r = expand_path(&yaml).unwrap().unwrap();
        assert_eq!(&*r, Path::new("/tmp/foo"));
    }

    #[test]
    fn expand_path_plain_rel_path() {
        let yaml = Yaml::String("foo/bar".into());
        let r = expand_path(&yaml).unwrap().unwrap();
        assert_eq!(&*r, Path::new("foo/bar"));
    }

    #[test]
    fn expand_path_home_expansion() {
        let yaml = Yaml::String("~/testdir".into());
        let r = expand_path(&yaml).unwrap().unwrap();
        let home = dirs::home_dir().unwrap();
        assert_eq!(*r, home.join("testdir"));
    }
}
