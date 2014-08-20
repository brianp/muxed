use std::io::fs;
use std::path::posix::Path;
use std::os::homedir;
#[cfg(test)] use test_helper::{cleanup_dir,random_name};

static MUXED_NAME_STR: &'static str = "muxed";

/// Create the muxed directory and return the path if creation is successful.
pub fn create_muxed_dir(name: &Path) -> Path {
    match fs::mkdir(path, ::std::io::UserRWX) {
        Ok(()) => (),
        Err(_e) => println!("Failed to create muxed directory: {}", path.filename())
    }

    path.clone()
}

/// Return the users current homedir as a string.
pub fn homedir_string() -> String {
    let home_unwrap = homedir().unwrap();
    format!("{}", home_unwrap.display())
}

pub fn muxed_dir() -> Path {
    match MUXED_DIR.exists() {
        true  => MUXED_DIR,
        false => create_muxed_dir(MUXED_DIR)
    }
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
    cleanup_dir(muxed_path);
}

#[test]
fn creates_muxed_dir() {
    let dir = random_name();
    let muxed_path  = &Path::new(format!("{}/.{}/", homedir_string(), dir));
    create_muxed_dir(&dir);
    assert!(muxed_path.exists());
    cleanup_dir(muxed_path);
}
