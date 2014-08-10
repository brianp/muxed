//! The initializer module parses arguments and verfies commands are valid
//! The creator module handles creating the muxed directory and project files.
#![allow(experimental)]

use std::io::File;
use std::path::posix::Path;
use editor;
use root;

#[cfg(test)] use test_helper::{random_name,cleanup_file};

static TEMPLATE: &'static str = include_str!("creator/template.toml");
static DEFAULT_MUXED_DIR: &'static str = "muxed";

pub fn new(name: &str) {
    let muxed_dir = match root::muxed_dir_exists(&DEFAULT_MUXED_DIR.to_string()) {
        true  => Path::new(format!("{}/.{}/", root::homedir_string(), &DEFAULT_MUXED_DIR.to_string())),
        false => root::create_muxed_dir(&DEFAULT_MUXED_DIR.to_string())
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

/// Copy and create the new project file from a template.
fn create_project_file(path: &Path) {
    let filename = project_filename(path);
    match File::create(path).write(modified_template(TEMPLATE, filename.as_slice()).as_bytes()) {
        Ok(())  => (),
        Err(_e) => println!("Failed to create project {}", path.filename()),
    }
}

fn modified_template(template: &str, project_name: &str) -> String {
    template.replace("{file_name}", project_name)
}

fn project_filename(path: &Path) -> String {
    String::from_utf8(path.filename().unwrap().to_vec()).unwrap().replace(".toml", "")
}

#[test]
fn populates_template_values() {
    let value = modified_template(TEMPLATE, "muxed project");
    let result = value.as_slice().contains("muxed project");
    assert!(result);
}

#[test]
fn replaces_template_values() {
    let value = modified_template(TEMPLATE, "muxed project");
    let result = !value.as_slice().contains("{file_name}");
    assert!(result);
}

#[test]
fn creates_project_file() {
    let path = &Path::new(format!("{}/.muxed/{}.toml", root::homedir_string(), random_name()));
    create_project_file(path);
    assert!(path.exists());

    cleanup_file(path);
}

#[test]
fn errors_when_creating_project_file() {
    //assert!(false);
}

#[test]
fn create_copies_the_template_file() {
    let path = &Path::new(format!("{}/.muxed/{}.toml", root::homedir_string(), random_name()));
    let filename = project_filename(path);
    create_project_file(path);
    let data = File::open(path).read_to_end().unwrap();
    let template_expectation = modified_template(TEMPLATE, filename.as_slice());
    assert_eq!(data.as_slice(), template_expectation.as_bytes());

    cleanup_file(path);
}

#[test]
fn new_writes_file_to_muxed_dir() {
    let name = random_name();
    let path = &Path::new(format!("{}/.muxed/{}.toml", root::homedir_string(), name));
    new(name.as_slice());
    assert!(path.exists());

    cleanup_file(path);
}
