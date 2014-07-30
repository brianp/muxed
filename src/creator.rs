#![allow(experimental)]

use std::io::{File,fs};
use std::path::posix::Path;
use std::os::{homedir};
use std::rand::random;
use std::finally::Finally;

static TEMPLATE: &'static str = include_str!("creator/template.yml");
static DEFAULT_MUXED_DIR: &'static str = "muxed";

pub fn new(name: &str) {
    let muxed_dir = if muxed_dir_exists(&DEFAULT_MUXED_DIR.to_string()) {
      Path::new(format!("{}/.{}/", homedir_string(), &DEFAULT_MUXED_DIR.to_string()))
    } else {
      create_muxed_dir(&DEFAULT_MUXED_DIR.to_string())
    };

    let path = &Path::new(format!("{}/{}", muxed_dir.display(), name));
    if !path.exists() {
      File::create(&Path::new(path));
    } else {
      println!("Project already exists.");
    }
}

fn create_muxed_dir(name: &String) -> Path {
    let path = &Path::new(format!("{}/.{}", homedir_string(), name));
    fs::mkdir(path, ::std::io::UserRWX);
    path.clone()
}

fn muxed_dir_exists(name: &String) -> bool {
    let path = &Path::new(format!("{}/.{}", homedir_string(), name));
    path.exists()
}

fn homedir_string() -> String {
    let home_unwrap = homedir().unwrap();
    format!("{}", home_unwrap.display())
}

#[test]
fn muxed_dir_exists_returns_false() {
  let dir = format!("test_dir_{}", random::<f64>());
  assert!(!muxed_dir_exists(&dir));
}

#[test]
fn muxed_dir_exists_returns_true() {
  let dir = format!("test_dir_{}", random::<f64>());
  create_muxed_dir(&dir);
  assert!(muxed_dir_exists(&dir));

  let muxed_path  = &Path::new(format!("{}/.{}/", homedir_string(), dir.as_slice()));
  fs::rmdir_recursive(muxed_path);
}

#[test]
fn creates_muxed_dir() {
    let name        = format!("test_project_{}", random::<f64>());
    let dir         = format!("test_dir_{}", random::<f64>());
    let muxed_path  = &Path::new(format!("{}/.{}/", homedir_string(), dir.as_slice()));
    create_muxed_dir(&dir);
    assert!(muxed_path.exists());
    fs::rmdir_recursive(muxed_path);
}

#[test]
fn new_writes_file_to_muxed_dir() {
    let name = format!("test_project_{}", random::<f64>());
    let path = &Path::new(format!("{}/.muxed/{}", homedir_string(), name));
    new(name.as_slice());
    assert!(path.exists());
    fs::unlink(path);
}

#[test]
#[should_fail]
fn new_doesnt_overwrite_existing_file() {
    let name = format!("test_project_{}", random::<f64>());
    let path = &Path::new(format!("{}/.muxed/{}", homedir_string(), name));
    new(name.as_slice());
    (|| {
        new(name.as_slice());
    }).finally(|| {
        fs::unlink(path);
    })
}
