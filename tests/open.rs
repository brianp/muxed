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
    use helpers::*;
    use std::thread::sleep;
    use std::time::Duration;

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
windows: ['vi', 'ls', 'git']
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
      panes: ['vi', 'ls']
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
      panes: ['vi', 'ls']
";
        let session = test_with_contents(contents);
        let num = session.windows.get("editor").unwrap().get("Panes").unwrap().to_owned();
        assert_eq!(num, 2)
    }

    #[test]
    fn multiple_windows_with_panes() {
        let contents = b"---
windows:
  - editor:
      layout: 'main-vertical'
      panes: ['vi', 'ls']
  - tests:
      layout: 'main-vertical'
      panes: ['vi', 'ls', 'ls']
";
        let session = test_with_contents(contents);
        let num = session.windows.get("editor").unwrap().get("Panes").unwrap().to_owned();
        let num1 = session.windows.get("tests").unwrap().get("Panes").unwrap().to_owned();
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
        let num = session.windows.get("editor").unwrap().get("Panes").unwrap().to_owned();
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
        let active_dir = session.active_dir;
        let _ = fs::remove_dir(dir);
        assert_eq!(active_dir, "/tmp/Directory With Spaces");
    }
}
