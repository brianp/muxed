use std::io::{USER_RWX,IoError};
use std::io::fs::{PathExtensions,mkdir};
#[cfg(test)] use test_helper::{cleanup_dir, random_name};

/// Copy and create the new project file from a template.
pub fn create(path: &Path) -> Result<(), IoError>{
    mkdir(path, USER_RWX)
}

#[test]
fn creates_muxed_dir() {
    let path    = &Path::new(format!("/tmp/{}", random_name()));
    let _result = create(path);
    assert!(path.exists());

    if path.exists() {
        cleanup_dir(path);
    }
}

#[test]
fn creates_muxed_dir_without_error () {
    let path   = &Path::new(format!("/tmp/{}", random_name()));
    let result = create(path);

    assert!(result.is_ok());
    if path.exists() {
        cleanup_dir(path);
    }
}

#[test]
fn errors_when_creating_muxed_dir() {
    let path   = &Path::new("");
    let result = create(path);
    assert!(result.is_err());
}
