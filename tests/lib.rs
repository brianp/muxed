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
use std::thread::sleep;
use std::time::Duration;

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
    let output = Command::new("./target/debug/muxed")
        .arg("-d")
        .arg(format!("{}", project))
        .output()
        .unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
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
    println!("{:?}", buffer);
    println!("exists? {:?}", path.exists());
    let _ = buffer.write(b"---
windows: ['cargo', 'vim', 'git']
");
    open_muxed(&format!("{}", name));
    let time = Duration::new(1, 0);
    sleep(time);
    let result = list_windows(&name.to_string());
    let _ = fs::remove_file(path);
    println!("{:?}", result);
    assert_eq!(result, "hi")
}
