//! The integration suite for inspecting sessions.

extern crate libc;
extern crate rand;

mod open {
    use std::process::Command;
    use rand::random;
    use std::fs::File;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::io::prelude::*;
    use std::env::home_dir;

    fn homedir() -> Result<PathBuf, String>{
        match home_dir() {
            Some(dir) => Ok(dir),
            None      => Err(String::from("We couldn't find your home directory."))
        }
    }

    /// List windows will give details about the active sessions in testing.
    /// target: A string represented by the {named_session}:{named_window}
    fn list_windows(target: &String) -> String {
        let output = Command::new("tmux")
                         .arg("list-windows")
                         .arg("-t")
                         .arg(target)
                         .output()
                         .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

        String::from_utf8_lossy(&output.stdout).into_owned()
    }

    fn open_muxed(project: &String) -> () {
        Command::new("./target/debug/muxed")
            .arg("-d")
            .arg(format!("{}", project))
            .output()
            .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    }

    fn kill_session(target: &String) -> () {
        Command::new("tmux")
            .arg("kill-session")
            .arg("-t")
            .arg(target)
            .output()
            .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    }

    fn has_n_windows(results: &Vec<&str>) -> usize {
        results.len() - 1
    }

    #[test]
    fn list_3_windows() {
        let name = random::<u16>();
        let home = homedir().unwrap();
        let name1 = format!("{}/.muxed/{}.yml", home.display(), name);
        let path = Path::new(&name1);
        let path1 = &format!("{}/.muxed/", home.display());
        let muxed_path = Path::new(path1);
        if !muxed_path.exists() { println!("{:?}", fs::create_dir(muxed_path)) };
        let mut buffer = File::create(path).unwrap();
        let _ = buffer.write(b"---
    windows: ['cargo', 'vim', 'git']
    ");
        open_muxed(&format!("{}", name));
        let result = list_windows(&name.to_string());
        let results: Vec<&str> = result.split("\n").collect();
        let _ = fs::remove_file(path);
        kill_session(&name.to_string());
        assert_eq!(has_n_windows(&results), 3)
    }
}
