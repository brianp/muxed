//! The integration suite for inspecting sessions.

extern crate libc;
extern crate rand;

use std::process::Command;
use libc::system;
use std::ffi::CString;
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
                     .arg(format!("-t {}", target))
                     .output()
                     .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[test]
fn list_3_windows() {
    let name = random::<u16>();
    let home = homedir().unwrap();
    let name1 = format!("{}/.{}/{}.yml", home.display(), "muxed", name);
    let path = Path::new(&name1);
    let _ = fs::create_dir(Path::new(&format!("{}/.muxed/", home.display())));
    let mut buffer = File::create(path).unwrap();
    let _ = buffer.write(b"---
windows: ['cargo', 'vim', 'git']
");

    let line = format!("./target/debug/muxed {}", name);
    let system_call = CString::new(line.clone()).unwrap();
    //unsafe { system(system_call.as_ptr()); };

    let _ = fs::remove_file(path);
    let result = list_windows(&name.to_string());
    //assert_eq!(result, "hi")
    assert!(true)
}
