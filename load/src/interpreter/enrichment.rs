//! The YAML parser. Here is where we convert the yaml in to commands to be
//! processed later.

use common::project_paths::homedir;
use common::tmux::session::{NodeMut, Session};
use common::tmux::{Config, Target};
use std::path::{Component, Path, PathBuf};

/// Enriches a `Session` with configuration details and contextual information,
/// preparing command stacks for user tmux sessions.
///
/// This function sets the session's name, configuration, target, and optionally
/// root and daemonization flags. It iterates through the session nodes,
/// updating pane and window targets and paths using the given project name
/// and adjusted index values from the config.
///
/// # Arguments
/// * `session` - The mutable tmux session to enrich.
/// * `project_name` - The name of the current project, used for target naming.
/// * `daemonize` - Whether the session should be daemonized.
/// * `config` - The project configuration containing base indices and other settings.
///
/// Paths for windows and panes are expanded as needed, and both window and pane
/// indices are offset according to the configuration values. Targets are assigned
/// reflecting the enriched state for tmux session startup.
pub fn enrich(session: &mut Session, project_name: String, daemonize: bool, config: Config) {
    let base_index = config.base_index;
    let pane_base_index = config.pane_base_index;

    session.name = Some(project_name.clone());
    session.config = Some(config);
    session.target = Some(Target::new(project_name.clone(), None, None));

    if daemonize {
        session.daemonize = Some(daemonize);
    }

    if let Some(root) = session.root.as_ref() {
        session.root = expand_path(root);
    }

    let root = session.root.clone();

    for node in session.iter_mut() {
        match node {
            NodeMut::Pane {
                window_index,
                pane_index,
                pane,
            } => {
                let adjusted_window_index = window_index + base_index;
                let adjusted_pane_index = pane_index + pane_base_index;

                pane.path = match pane.path.as_ref() {
                    Some(path) => expand_path(path),
                    None => root.clone(),
                };

                pane.target = Some(Target::new(
                    project_name.clone(),
                    Some(adjusted_window_index),
                    Some(adjusted_pane_index),
                ));
            }
            NodeMut::Window { index, window } => {
                let adjusted_index = index + base_index;

                window.path = match window.path.as_ref() {
                    Some(path) => expand_path(path),
                    None => root.clone(),
                };

                window.target = Some(Target::new(
                    project_name.clone(),
                    Some(adjusted_index),
                    None,
                ));
            }
        }
    }
}

/// Expands a given path, replacing a leading `~` with the user's home directory if present.
///
/// If the path starts with `~`, this function attempts to resolve it to the current user's
/// home directory and append any additional subpaths. If the path does not begin with `~`,
/// it is returned unchanged. Returns `None` if home directory resolution fails.
fn expand_path(path: &Path) -> Option<PathBuf> {
    let mut components = path.components();

    match components.next() {
        Some(Component::Normal(os_str)) if os_str == "~" => {
            if let Some(home) = homedir() {
                let mut result = home;

                for c in components {
                    result.push(c.as_os_str());
                }

                Some(result)
            } else {
                None
            }
        }
        _ => Some(path.to_path_buf()),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use common::tmux::{Config, Target};
    use common::tmux::{Pane, Session, Window};
    use std::path::PathBuf;

    fn test_config() -> Config {
        Config {
            base_index: 1,
            pane_base_index: 2,
        }
    }

    fn basic_session(root: Option<PathBuf>) -> Session {
        Session {
            name: None,
            windows: vec![Window {
                name: "alpha".to_string(),
                panes: vec![
                    Pane {
                        path: root.clone(),
                        ..Default::default()
                    },
                    Pane {
                        path: None,
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }],
            root,
            ..Default::default()
        }
    }

    #[test]
    fn assigns_project_name_and_config() {
        let mut sess = Session::default();
        let conf = test_config();
        enrich(&mut sess, "projA".into(), false, conf.clone());
        assert_eq!(sess.name.as_deref(), Some("projA"));
        assert_eq!(sess.config.as_ref(), Some(&conf));
    }

    #[test]
    fn sets_daemonize_field_true_or_none() {
        let mut sess = Session::default();
        enrich(&mut sess, "proj".into(), true, Config::default());
        assert_eq!(sess.daemonize, Some(true));

        let mut sess2 = Session::default();
        enrich(&mut sess2, "proj2".into(), false, Config::default());
        assert_eq!(sess2.daemonize, None);
    }

    #[test]
    fn expands_root_tilde_to_home() {
        let home = homedir().unwrap();
        let tilde = "~/my-root-folder";
        let mut sess = basic_session(Some(PathBuf::from(tilde)));
        enrich(&mut sess, "x".into(), false, Config::default());
        let expected = home.join("my-root-folder");
        assert_eq!(sess.root, Some(expected));
    }

    #[test]
    fn leaves_absolute_and_relative_paths_unmodified() {
        // Absolute path: should remain unchanged
        let abs = "/tmp/abspath";
        let mut sess = basic_session(Some(PathBuf::from(abs)));
        enrich(&mut sess, "z".into(), false, Config::default());
        assert_eq!(sess.root, Some(PathBuf::from(abs)));

        // Relative path: should remain unchanged
        let rel = "foo/bar";
        let mut sess2 = basic_session(Some(PathBuf::from(rel)));
        enrich(&mut sess2, "y".into(), false, Config::default());
        assert_eq!(sess2.root, Some(PathBuf::from(rel)));
    }

    #[test]
    fn expands_pane_and_window_paths() {
        let home = homedir().unwrap();
        let tilde = "~/stuff";
        let root = PathBuf::from(tilde);

        // Pane with tilde path, window/path none (should fallback to root)
        let mut sess = basic_session(Some(root));
        enrich(&mut sess, "proj".into(), false, Config::default());

        let expected = home.join("stuff");
        // Pane[0] path was tilde, should be expanded
        assert_eq!(sess.windows[0].panes[0].path, Some(expected.clone()));
        // Pane[1] path was None, should now be set to root
        assert_eq!(sess.windows[0].panes[1].path, Some(expected));
    }

    #[test]
    fn sets_targets_on_windows_and_panes_with_config_base_indices() {
        let mut sess = basic_session(Some(PathBuf::from("abc")));
        let conf = Config {
            base_index: 7,
            pane_base_index: 3,
        };
        enrich(&mut sess, "thing".into(), false, conf);

        // The target index reflects config.base_index & pane_base_index
        let window_target = &sess.windows[0].target;
        assert_eq!(window_target.as_ref().unwrap().session, "thing");
        assert_eq!(window_target.as_ref().unwrap().window, Some(7));

        let pane0_target = &sess.windows[0].panes[0].target;
        assert_eq!(pane0_target.as_ref().unwrap().session, "thing");
        assert_eq!(pane0_target.as_ref().unwrap().window, Some(7));
        assert_eq!(pane0_target.as_ref().unwrap().pane, Some(3));
    }

    #[test]
    fn preserves_existing_targets_if_present() {
        let mut sess = basic_session(Some(PathBuf::from("root")));
        sess.windows[0].target = Some(Target::new("custom".to_string(), Some(123), None));
        sess.windows[0].panes[0].target = Some(Target::new("pre".to_string(), Some(1), Some(99)));
        enrich(&mut sess, "repl".into(), false, Config::default());

        assert_eq!(sess.windows[0].target.as_ref().unwrap().session, "repl");
        assert_eq!(
            sess.windows[0].panes[0].target.as_ref().unwrap().session,
            "repl"
        );
    }

    #[test]
    fn no_root_means_no_path_expansion() {
        let mut sess = basic_session(None);
        enrich(&mut sess, "proj".into(), false, Config::default());
        // If no root, pane and window path remain None (unless previously set)
        assert_eq!(sess.windows[0].panes[0].path, None);
        assert_eq!(sess.windows[0].panes[1].path, None);
        assert_eq!(sess.windows[0].path, None);
    }

    #[test]
    fn works_with_multiple_windows_and_panes() {
        let mut sess = Session {
            name: None,
            windows: vec![
                Window {
                    name: "w0".to_string(),
                    panes: vec![Pane::default()],
                    ..Default::default()
                },
                Window {
                    name: "w1".to_string(),
                    panes: vec![
                        Pane {
                            path: Some(PathBuf::from("~/bin")),
                            ..Default::default()
                        },
                        Pane::default(),
                    ],
                    ..Default::default()
                },
            ],
            root: Some(PathBuf::from("/tmp/abc")),
            ..Default::default()
        };
        enrich(&mut sess, "xy".into(), false, Config::default());

        // Window0/pane0 path will be from root
        assert_eq!(
            sess.windows[0].panes[0].path,
            Some(PathBuf::from("/tmp/abc"))
        );
        // Window1/pane0 path will be expanded
        let home_bin = homedir().unwrap().join("bin");
        assert_eq!(sess.windows[1].panes[0].path, Some(home_bin));
    }

    #[test]
    fn session_fields_unchanged_if_no_op() {
        let mut sess = Session {
            name: Some("noop".to_string()),
            root: None,
            ..Default::default()
        };
        let orig = sess.clone();
        enrich(&mut sess, "noop".to_string(), false, Config::default());
        // Since names match and nothing else set, session stays the same
        assert_eq!(sess.name, orig.name);
        assert_eq!(sess.root, orig.root);
    }

    #[cfg(test)]
    mod expand_path_tests {
        use super::*;
        use std::path::PathBuf;

        fn mock_home() -> PathBuf {
            // Use the actual homedir, but you can modify this for more control in the future
            homedir().expect("Homedir should exist for test")
        }

        #[test]
        fn expands_tilde_to_home() {
            let input = PathBuf::from("~");
            let result = expand_path(&input);
            assert_eq!(result, Some(mock_home()));
        }

        #[test]
        fn expands_tilde_prefix_with_subdirs() {
            let input = PathBuf::from("~/some/folder");
            let result = expand_path(&input);
            let mut expected = mock_home();
            expected.push("some");
            expected.push("folder");
            assert_eq!(result, Some(expected));
        }

        #[test]
        fn leaves_absolute_path_unmodified() {
            let input = PathBuf::from("/usr/local/bin");
            let result = expand_path(&input);
            assert_eq!(result, Some(PathBuf::from("/usr/local/bin")));
        }

        #[test]
        fn leaves_relative_path_unmodified() {
            let input = PathBuf::from("foo/bar/baz");
            let result = expand_path(&input);
            assert_eq!(result, Some(PathBuf::from("foo/bar/baz")));
        }
    }
}
