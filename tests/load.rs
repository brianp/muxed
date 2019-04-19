//! The integration suite for inspecting sessions.

extern crate dirs;
extern crate libc;
extern crate rand;
extern crate regex;
extern crate yaml_rust;

mod helpers;

mod load {
    use dirs::home_dir;
    use helpers::*;
    use rand::random;
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::PathBuf;
    use std::str;
    use yaml_rust::YamlLoader;

    fn project_name(contents: &[u8]) -> String {
        let string_content = str::from_utf8(contents).unwrap();
        let yaml = YamlLoader::load_from_str(string_content).unwrap();

        match yaml[0]["name"].as_str() {
            Some(x) => x.to_string(),
            None => format!("muxed_int_test_{}", random::<u16>()),
        }
    }

    fn setup(contents: &[u8]) -> (String, PathBuf) {
        let project_name = project_name(contents);
        let project_file = format!("/tmp/muxed_{}/{}.yml", random::<u16>(), project_name);
        let project_path = PathBuf::from(&project_file);

        let muxed_path = project_path.parent().unwrap();
        if !muxed_path.exists() {
            println!("{:?}", fs::create_dir(muxed_path))
        };

        let mut buffer = File::create(&project_path).unwrap();
        let _ = buffer.write(contents);

        (project_name, project_path.clone())
    }

    fn cleanup(project_name: &str, config_path: &PathBuf) -> () {
        let _ = fs::remove_file(config_path);
        let _ = fs::remove_dir(config_path.parent().unwrap());
        kill_session(project_name);
    }

    fn test_with_contents(contents: &[u8]) -> TmuxSession {
        let (project_name, config_path) = setup(contents);
        open_muxed(&project_name, config_path.parent().unwrap());

        let completed = PathBuf::from(format!("/tmp/{}.complete", project_name));
        let exec = format!("touch '{}'", completed.display());

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
        let num = session.windows["editor"].panes.as_usize().unwrap();
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
        let num = session.windows["editor"].panes.as_usize().unwrap();
        let num1 = session.windows["tests"].panes.as_usize().unwrap();
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
        let num = session.windows["editor"].panes.as_usize().unwrap();
        assert_eq!(num, 2)
    }

    #[test]
    fn expect_to_open_in_directory_containing_spaces() {
        let dir = PathBuf::from("/tmp/Directory With Spaces/");
        if !dir.exists() {
            println!("{:?}", fs::create_dir(&dir))
        };
        let contents = b"---
root: /tmp/Directory With Spaces/
windows:
  - editor: ''
";
        let session = test_with_contents(contents);
        let pane_current_path = session.windows["editor"]
            .pane_current_path
            .as_str()
            .unwrap();
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
        let pane_current_path = session.windows["editor"]
            .pane_current_path
            .as_str()
            .unwrap();
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
        let pane_current_path = session.windows["editor"]
            .pane_current_path
            .as_str()
            .unwrap();
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

    #[test]
    fn expect_pre_to_create_file() {
        let file = PathBuf::from(format!("/tmp/{}", random::<u16>()));
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
        let file1 = PathBuf::from(format!("/tmp/{}", random::<u16>()));
        let file2 = PathBuf::from(format!("/tmp/{}", random::<u16>()));
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
        let file = PathBuf::from(format!("/tmp/{}", random::<u16>()));
        let contents = format!(
            "---
pre_window: echo 'pre_window' >> {}
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
        let file = PathBuf::from(format!("/tmp/{}", random::<u16>()));
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
        let name = session.name.as_str().unwrap();
        assert_eq!(name, "Brians Session")
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
