//! The integration suite for inspecting sessions.

extern crate libc;
extern crate rand;
extern crate regex;

mod helpers;

mod open {
    use rand::{random};
    use std::fs::File;
    use std::fs;
    use std::path::PathBuf;
    use std::io::prelude::*;
    use std::env::home_dir;
    use helpers::*;

    fn setup(contents: &'static [u8]) -> (String, PathBuf) {
        let project_name = format!("muxed_int_test_{}", random::<u16>());
        let project_file = format!("/tmp/muxed_{}/{}.yml", random::<u16>(), project_name);
        let project_path = PathBuf::from(&project_file);

        let muxed_path = project_path.parent().unwrap();
        if !muxed_path.exists() { println!("{:?}", fs::create_dir(muxed_path)) };

        let mut buffer = File::create(&project_path).unwrap();
        let _ = buffer.write(contents);

        (project_name, project_path.clone())
    }

    fn cleanup(project_name: &String, config_path: &PathBuf) -> () {
        let _ = fs::remove_file(config_path);
        let _ = fs::remove_dir(config_path.parent().unwrap());
        kill_session(&project_name);
    }

    fn test_with_contents(contents: &'static [u8]) -> TmuxSession {
        let (project_name, config_path) = setup(contents);
        open_muxed(&project_name, config_path.parent().unwrap());
        let completed = PathBuf::from(format!("/tmp/{}.complete", project_name));
        let exec = format!("touch {}", completed.display());
        send_keys(&project_name, &exec);
        wait_on(&completed);
        let session = TmuxSession::from_string(&list_windows(&project_name.to_string()));
        cleanup(&project_name, &config_path);
        session
    }

    #[test]
    fn opens_3_windows_from_array() {
        let contents = b"---
windows: ['ls', 'vi', 'git']
";
        let session = test_with_contents(contents);
        assert_eq!(session.num_of_windows, 3)
    }

    #[test]
    fn opens_2_windows() {
        let contents = b"---
windows:
  - editor:
      layout: 'main-vertical'
      panes: ['ls', 'vi']
  - stuff: ''
";
        let session = test_with_contents(contents);
        assert_eq!(session.num_of_windows, 2)
    }

    #[test]
    fn opens_3_windows_with_integer_names() {
        let contents = b"---
windows: [1, 'ls', 3]
";
        let session = test_with_contents(contents);
        assert_eq!(session.num_of_windows, 3)
    }

    #[test]
    fn single_window_has_2_panes() {
        let contents = b"---
windows:
  - editor:
      layout: 'main-vertical'
      panes: ['ls', 'vi']
";
        let session = test_with_contents(contents);
        let num = session.windows.get("editor").unwrap().panes.as_usize().unwrap();
        assert_eq!(num, 2)
    }

    #[test]
    fn multiple_windows_with_panes() {
        let contents = b"---
windows:
  - editor:
      layout: 'main-vertical'
      panes: ['ls', 'vi']
  - tests:
      layout: 'main-vertical'
      panes: ['ls', 'vi', 'ls']
";
        let session = test_with_contents(contents);
        let num = session.windows.get("editor").unwrap().panes.as_usize().unwrap();
        let num1 = session.windows.get("tests").unwrap().panes.as_usize().unwrap();
        assert_eq!(num, 2);
        assert_eq!(num1, 3)
    }

    #[test]
    fn window_with_empty_command_is_valid() {
        let contents = b"---
windows:
  - editor:
";
        let session = test_with_contents(contents);
        assert_eq!(session.num_of_windows, 1)
    }

    #[test]
    fn panes_with_empty_commands_are_valid() {
        let contents = b"---
windows:
  - editor:
      layout: 'main-vertical'
      panes:
        -
        -
";
        let session = test_with_contents(contents);
        let num = session.windows.get("editor").unwrap().panes.as_usize().unwrap();
        assert_eq!(num, 2)
    }

    #[test]
    fn expect_to_open_in_directory_containing_spaces() {
        let dir = PathBuf::from("/tmp/Directory With Spaces/");
        if !dir.exists() { println!("{:?}", fs::create_dir(&dir)) };
        let contents = b"---
root: /tmp/Directory With Spaces/
windows:
  - editor: ''
";
        let session = test_with_contents(contents);
        let pane_current_path = session.windows.get("editor").unwrap().pane_current_path.as_str().unwrap();
        let _ = fs::remove_dir(dir);
        // Use contains because OSX on travis ci symlinks /tmp/ to /private/tmp/
        // resulting in `pane_current_path` being `/private/tmp/Direct…`
        assert!(pane_current_path.contains("/tmp/Directory With Spaces"));
    }

    #[test]
    fn expect_home_var_to_open_in_home_dir() {
        let contents = b"---
root: '$HOME'
windows:
  - editor: ''
";
        let session = test_with_contents(contents);
        let pane_current_path = session.windows.get("editor").unwrap().pane_current_path.as_str().unwrap();
        assert_eq!(pane_current_path, home_dir().unwrap().to_str().unwrap());
    }

    #[test]
    fn expect_tilde_slash_to_open_in_home_dir() {
        let contents = b"---
root: ~/
windows:
  - editor: ''
";
        let session = test_with_contents(contents);
        let pane_current_path = session.windows.get("editor").unwrap().pane_current_path.as_str().unwrap();
        assert_eq!(pane_current_path, home_dir().unwrap().to_str().unwrap());
    }

    #[test]
    fn expect_focus_on_the_first_window() {
        let contents = b"---
windows: ['ssh', 'git']
";
        let session = test_with_contents(contents);
        let window_active = session.window_active.as_str().unwrap();
        assert_eq!(window_active, "ssh")
    }

// This test should exist but we currently don't do anything to list panes.
//    #[test]
//    fn expect_focus_on_the_top_pane() {
//        let contents = b"---
//windows:
//  - ssh:
//    layout: main-horizontal
//    panes:
//      - ''
//      - ''
//  - git: ''
//";
//        let session = test_with_contents(contents);
//        assert_eq!(session.pane_active, "ssh.0")
//    }
}
