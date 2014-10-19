use std::io::{File, IoError};
use std::io::fs::PathExtensions;
use libc::funcs::c95::stdlib::system;
#[cfg(test)] use test_helper::{cleanup_file, random_name};

/// Copy and create the new project file from a template.
pub fn create(path: &Path, content: &str) -> Result<(), IoError>{
    File::create(path).write(content.as_bytes())
}

/// Open a project file with the default editor. Uses C directly to interact
/// with the system. This method is overloaded below for the test config to not
/// execture during testing.
#[cfg(not(test))] pub fn open(path: &Path) {
    let method = format!("$EDITOR {}", path.display()).to_c_str();
    unsafe { system(method.unwrap()); };
}

/// Overloaded method for use in testing. Doesn't do anything at all.
#[cfg(test)] pub fn open(_path: &Path) { }

#[test]
fn creates_file() {
    let path = &Path::new(format!("/tmp/project_file_{}.toml", random_name()));
    create(path, "");
    assert!(path.exists());

    cleanup_file(path);
}

#[test]
fn creates_file_with_content() {
    let path    = &Path::new(format!("/tmp/project_file_{}.toml", random_name()));
    let content = "this content";
    create(path, content);

    let file_contents = File::open(path).read_to_end().unwrap();
    let read_contents = String::from_utf8(file_contents);

    assert_eq!(content, read_contents.unwrap().as_slice());

    cleanup_file(path);
}

#[test]
fn creates_files_without_error () {
    let path   = &Path::new(format!("/tmp/project_file_{}.toml", random_name()));
    let result = create(path, "");

    assert!(result.is_ok());
    cleanup_file(path);
}

#[test]
fn errors_when_creating_project_file() {
    let path = &Path::new("");
    let result = create(path, "");
    assert!(result.is_err());
}
