#![allow(experimental)]

use std::io::{File,fs};
//use std::io::process::Command;
use std::path::posix::Path;
use std::os::{homedir};
#[cfg(test)] use std::rand::random;
#[cfg(test)] use std::finally::Finally;

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
      create_project_file(path)
    } else {
      println!("Project already exists.");
    }
}

fn create_project_file(path: &Path) {
    match File::create(path).write(TEMPLATE.as_bytes()) {
        Ok(()) => (), // succeeded
        Err(_e) => println!("Failed to create project {}", path.filename()),
    }
//    Command::new("vim").arg(format!("{}", path.display())).detached();
}

fn create_muxed_dir(name: &String) -> Path {
    let path = &Path::new(format!("{}/.{}", homedir_string(), name));
    match fs::mkdir(path, ::std::io::UserRWX) {
        Ok(()) => (), // succeeded
        Err(_e) => println!("Failed to create project {}", path.filename()),
    }

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

#[cfg(test)]
fn random_name() -> String {
  format!("test_{}", random::<f64>())
}

#[test]
fn muxed_dir_exists_returns_false() {
  assert!(!muxed_dir_exists(&random_name()));
}

#[test]
fn muxed_dir_exists_returns_true() {
  let dir = random_name();
  create_muxed_dir(&dir);
  assert!(muxed_dir_exists(&dir));

  let muxed_path  = &Path::new(format!("{}/.{}/", homedir_string(), dir));
  match fs::rmdir_recursive(muxed_path) {
      Ok(()) => (), // succeeded
      Err(e) => println!("Failed to remove the path {} with error {}", muxed_path.display(), e),
  }
}

#[test]
fn creates_muxed_dir() {
    let dir = random_name();
    let muxed_path  = &Path::new(format!("{}/.{}/", homedir_string(), dir));
    create_muxed_dir(&dir);
    assert!(muxed_path.exists());
    match fs::rmdir_recursive(muxed_path) {
        Ok(()) => (), // succeeded
        Err(e) => println!("Failed to remove the path {} with error {}", muxed_path.display(), e),
    }
}

#[test]
fn new_writes_file_to_muxed_dir() {
    let name = random_name();
    let path = &Path::new(format!("{}/.muxed/{}", homedir_string(), name));
    new(name.as_slice());
    assert!(path.exists());
    match fs::unlink(path) {
        Ok(()) => (), // succeeded
        Err(e) => println!("Failed to unlink the path {} with error {}", path.display(), e),
    }
}

#[test]
// TODO: Fix this test so it verifies something better.
fn new_doesnt_overwrite_existing_file() {
    let name = random_name();
    let path = &Path::new(format!("{}/.muxed/{}", homedir_string(), name));
    new(name.as_slice());
    (|| {
        new(name.as_slice());
    }).finally(|| {
        match fs::unlink(path) {
            Ok(()) => (), // succeeded
            Err(e) => println!("Failed to unlink the path {} with error {}", path.display(), e),
        }
    })
}
