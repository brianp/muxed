//! The initializer module parses arguments and verfies commands are valid
//! The creator module handles creating the muxed directory and project files.
#![allow(experimental)]

use std::io::File;
use std::path::posix::Path;
use editor;
use root;

#[cfg(test)] use test_helper::{random_name,cleanup_file,cleanup_dir};

static TEMPLATE: &'static str = include_str!("creator/template.toml");

pub fn new(name: &str) {
    let muxed_dir = root::muxed_dir();
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
        Err(_e) => println!("Failed to create project {}", filename),
    }
}

fn modified_template(template: &str, project_name: &str) -> String {
    template.replace("{file_name}", project_name)
}

fn project_filename(path: &Path) -> String {
    String::from_utf8(path.filename().unwrap().to_vec()).unwrap().replace(".toml", "")
}

#[test]
fn returns_filename_as_string() {
  let path = &Path::new(format!("{}/{}", "luke", "skywalker"));
  assert_eq!(project_filename(path), String::from_str("skywalker"))
}

#[test]
fn accepts_periods_in_filenames() {
  let path = &Path::new(format!("{}/{}", "luke", "skywalker1.2"));
  assert_eq!(project_filename(path), String::from_str("skywalker1.2"))
}

#[test]
fn accepts_dashes_in_filenames() {
  let path = &Path::new(format!("{}/{}", "luke", "skywalker1-2"));
  assert_eq!(project_filename(path), String::from_str("skywalker1-2"))
}

#[test]
fn populates_template_values() {
    let value = modified_template(TEMPLATE, "muxed project");
    let result = value.as_slice().contains("muxed project");
    assert!(result);
}

#[test]
fn removes_template_placeholders() {
    let value = modified_template(TEMPLATE, "muxed project");
    let result = !value.as_slice().contains("{file_name}");
    assert!(result);
}

#[test]
fn creates_project_file() {
    let muxed_dir = &root::muxed_dir();
    let path = &Path::new(format!("{}/{}.toml", muxed_dir.display(), random_name()));
    create_project_file(path);
    assert!(path.exists());

    cleanup_dir(muxed_dir);
}

//#[test]
//fn errors_when_creating_project_file() {
//    //assert!(false);
//}

#[test]
fn create_copies_the_template_file() {
    let muxed_dir = &root::muxed_dir();
    let path = &Path::new(format!("{}/{}.toml", muxed_dir.display(), random_name()));
    let filename = project_filename(path);
    create_project_file(path);
    let data = File::open(path).read_to_end().unwrap();
    let template_expectation = modified_template(TEMPLATE, filename.as_slice());
    assert_eq!(data.as_slice(), template_expectation.as_bytes());

    cleanup_dir(muxed_dir);
}

#[test]
fn new_writes_file_to_muxed_dir() {
    let name = random_name();
    let muxed_dir = root::muxed_dir();
    let path = &Path::new(format!("{}/{}", muxed_dir.display(), name));

    println!("{}", path.display());

    new(name.as_slice());
    assert!(path.exists());

    cleanup_dir(path);
}
