//! The initializer module parses arguments and verfies commands are valid
//! The creator module handles creating the muxed directory and project files.
#![allow(experimental)]

use std::io::File;
use std::path::posix::Path;
use std::os::homedir;
use editor;
use root;
#[cfg(test)] use std::io::fs;
#[cfg(test)] use test_helper::random_name;

static TEMPLATE: &'static str = include_str!("creator/template.toml");
static DEFAULT_MUXED_DIR: &'static str = "muxed";

pub fn new(name: &str) {
    let muxed_dir = match muxed_dir_exists(&DEFAULT_MUXED_DIR.to_string()) {
        true  => Path::new(format!("{}/.{}/", homedir_string(), &DEFAULT_MUXED_DIR.to_string())),
        false => create_muxed_dir(&DEFAULT_MUXED_DIR.to_string())
    };

    let path = &Path::new(format!("{}/{}.toml", muxed_dir.display(), name));
    if !path.exists() {
        create_project_file(path);

        match editor::default_editor_set() {
            true  => editor::open_project_file(path),
            false => println!("Default editor is not set. Please define $EDITOR in your ~/.bashrc or similar file.")
        }
    } else {
        println!("Project already exists.");
    }
}

/// Copy and create the new project file from a template. Attempt to open the
/// users default editor to make changes.
fn create_project_file(path: &Path) {
    match File::create(path).write(TEMPLATE.as_bytes()) {
        Ok(()) => (),
        Err(_e) => println!("Failed to create project {}", path.filename()),
    }
}

#[test]
fn new_writes_file_to_muxed_dir() {
    let name = random_name();
    let path = &Path::new(format!("{}/.muxed/{}.toml", homedir_string(), name));
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
