//! The integration suite for inspecting sessions.

extern crate common;
extern crate snapshot;

extern crate dirs;
extern crate libc;
extern crate load;
extern crate rand;
extern crate retry_test;
extern crate yaml_rust;

mod helpers;

#[cfg(test)]
mod test {
    mod load {
        use crate::helpers::test_with_contents;
        use common::project_paths::homedir;
        use common::rand_names;
        use dirs::home_dir;
        use retry_test::retry_test;
        use std::env::temp_dir;
        use std::fs;
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::PathBuf;

        #[test]
        fn opens_3_windows_from_array() {
            let contents = b"---
windows: ['ls', 'vi', 'git']
";
            let session = test_with_contents(contents);
            assert_eq!(session.windows.len(), 3)
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
            assert_eq!(session.windows.len(), 2)
        }

        #[test]
        fn opens_3_windows_with_integer_names() {
            let contents = b"---
windows: [1, 'ls', 3]
";
            let session = test_with_contents(contents);
            assert_eq!(session.windows.len(), 3)
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
            let window = session.find_window_by_name("editor").unwrap();
            assert_eq!(window.panes.len(), 2)
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
            let editor_window = session.find_window_by_name("editor").unwrap();
            let tests_window = session.find_window_by_name("tests").unwrap();
            assert_eq!(editor_window.panes.len(), 2);
            assert_eq!(tests_window.panes.len(), 3)
        }

        #[test]
        fn window_with_empty_command_is_valid() {
            let contents = b"---
windows:
  - editor:
";
            let session = test_with_contents(contents);
            assert_eq!(session.windows.len(), 1)
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
            let window = session.find_window_by_name("editor").unwrap();
            assert_eq!(window.panes.len(), 2)
        }

        #[test]
        fn expect_to_open_in_directory_containing_spaces() {
            let dir = PathBuf::from("/tmp/Directory With Spaces/");
            if !dir.exists() {
                fs::create_dir(&dir).unwrap();
            };
            let contents = b"---
root: /tmp/Directory With Spaces/
windows:
  - editor: ''
";
            let session = test_with_contents(contents);
            let window = session.find_window_by_name("editor").unwrap();
            let pane = &window.panes[0];

            let right = dir.canonicalize().ok();
            let _ = fs::remove_dir(dir);
            // Use contains because OSX on travis ci symlinks /tmp/ to /private/tmp/
            // resulting in `pane_current_path` being `/private/tmp/Direct…`
            assert_eq!(pane.path, right);
        }

        #[test]
        #[retry_test(3, 10)]
        fn expect_home_var_to_open_in_home_dir() {
            let contents = b"---
root: '$HOME'
windows:
  - editor: ''
";
            let session = test_with_contents(contents);
            let window = session.find_window_by_name("editor").unwrap();
            let pane = &window.panes[0];
            assert_eq!(pane.path, homedir());
        }

        #[test]
        fn expect_tilde_slash_to_open_in_home_dir() {
            let contents = b"---
root: ~/
windows:
  - editor: ''
";
            let session = test_with_contents(contents);
            let window = session.find_window_by_name("editor").unwrap();
            let pane = &window.panes[0];
            assert_eq!(pane.path, home_dir());
        }

        #[test]
        #[retry_test(3, 10)]
        fn expect_window_path_to_take_priority() {
            let file = rand_names::project_file_in_tmp_dir();
            let window_path = file.parent().unwrap();
            let contents = format!(
                "---
root: ~/
windows:
  - editor:
      panes:
        - ls
      path: {}
",
                window_path.display()
            );
            let session = test_with_contents(contents.as_bytes());
            let window = session.find_window_by_name("editor").unwrap();
            let pane = &window.panes[0];

            assert_eq!(window_path.canonicalize().ok(), pane.path);
        }

        #[test]
        #[retry_test(3, 10)]
        fn expect_root_path_on_default_session_window_when_not_specified() {
            let contents = b"---
root: ~/
windows:
  - editor:
      panes:
        - ls
";
            let session = test_with_contents(contents);
            let window = session.find_window_by_name("editor").unwrap();
            let pane = &window.panes[0];

            assert_eq!(homedir(), pane.path);
        }

        #[test]
        fn first_window_path_shouldnt_be_default_path() {
            let file = rand_names::project_file_in_tmp_dir();
            let window_path = file.parent().unwrap();
            let contents = format!(
                "---
root: ~/
windows:
  - editor:
      panes:
        - ls
      path: {}
  - other: pwd
",
                window_path.display()
            );
            let session = test_with_contents(contents.as_bytes());
            let window = session.find_window_by_name("editor").unwrap();
            let pane = &window.panes[0];

            assert_eq!(temp_dir().canonicalize().ok(), pane.path);

            let window = session.find_window_by_name("other").unwrap();
            dbg!(session);
            let pane = &window.panes[0];

            assert_eq!(home_dir(), pane.path);
        }

        // TODO: should ssh or git be a command or window name?
        #[test]
        fn expect_focus_on_the_first_window() {
            let contents = b"---
windows: ['ssh', 'git']
";
            let session = test_with_contents(contents);
            let first_window = &session.windows[0];
            let other_window = &session.windows[1];

            assert!(first_window.active);
            assert_eq!(other_window.active, false)
        }

        #[test]
        fn expect_pre_to_create_file() {
            let file = rand_names::project_file_in_tmp_dir();
            let contents = format!(
                "---
pre: touch {}
windows: ['ssh', 'git']
",
                file.display()
            );

            let _ = test_with_contents(contents.as_bytes());
            assert!(file.exists());
            let _ = fs::remove_file(file);
        }

        #[test]
        fn expect_pre_to_create_two_files() {
            let file1 = rand_names::project_file_in_tmp_dir();
            let file2 = rand_names::project_file_in_tmp_dir();
            let contents = format!(
                "---
pre:
  - touch {}
  - touch {}
windows: ['ssh', 'git']
",
                file1.display(),
                file2.display()
            );

            let _ = test_with_contents(contents.as_bytes());
            assert!(file1.exists());
            assert!(file2.exists());
            let _ = fs::remove_file(file1);
            let _ = fs::remove_file(file2);
        }

        #[test]
        fn expect_pre_window_to_be_called_for_each_window() {
            let file = rand_names::project_file_in_tmp_dir();
            let contents = format!(
                "---
pre_window: \"echo 'pre_window' >> {}\"
windows: ['ssh', 'git']
",
                file.display()
            );
            let _ = test_with_contents(contents.as_bytes());
            let mut f = File::open(&file).unwrap();
            let mut s = String::new();
            let _ = f.read_to_string(&mut s);
            assert_eq!(s.lines().count(), 2);
            let _ = fs::remove_file(&file);
        }

        #[test]
        fn expect_pre_window_to_be_called_twice_for_each_window() {
            let file = rand_names::project_file_in_tmp_dir();
            let contents = format!(
                "---
pre_window:
  - echo 'pre_window' >> {}
  - echo 'pre_window' >> {}
windows: ['ssh', 'git']
",
                file.display(),
                file.display()
            );
            let _ = test_with_contents(contents.as_bytes());
            let mut f = File::open(&file).unwrap();
            let mut s = String::new();
            let _ = f.read_to_string(&mut s);
            assert_eq!(s.lines().count(), 4);
            let _ = fs::remove_file(&file);
        }

        #[test]
        fn expect_session_name_brians_session() {
            let contents = b"---
name: 'Brians Session'
windows: ['ssh', 'git']
";
            let session = test_with_contents(contents);
            assert_eq!(session.name.unwrap(), "Brians Session")
        }

        // TODO: Fix
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

        #[test]
        fn expect_attach_to_session_with_space_in_name() {
            let contents = "---
name: name with spaces
windows: ['ls']
"
            .to_string();
            let session = test_with_contents(contents.as_bytes());
            assert_eq!(session.name.unwrap(), "name with spaces");
        }
    }
}
