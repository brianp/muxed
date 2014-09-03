use std::io::fs;
use std::path::posix::Path;
use std::os::homedir;
#[cfg(test)] use test_helper::{cleanup_dir,random_name};

static MUXED_NAME_STR: &'static str = "muxed";

/// Create the muxed directory and return the path if creation is successful.
pub fn create_muxed_dir(path: &Path) -> Path {
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
    let path = muxed_path();
    match path.exists() {
        true  => path,
        false => create_muxed_dir(&path)
    }
}

#[cfg(test)] pub fn muxed_path() -> Path {
    Path::new(format!("/tmp/.muxed_{}/", random_name()))
}

#[cfg(not_test)] pub fn muxed_path() -> Path {
    Path::new(format!("{}/.{}/", root::homedir_string(), &MUXED_NAME_STR.to_string()))
}

#[test]
fn creates_muxed_dir() {
    let muxed_path = &muxed_path();
    create_muxed_dir(muxed_path);
    assert!(muxed_path.exists());
    cleanup_dir(muxed_path);
}

#[test]
fn muxed_dir_creates_dir() {
    let muxed_path = &muxed_dir();
    assert!(muxed_path.exists());
    cleanup_dir(muxed_path);
}

//#[test]
//fn muxed_dir_finds_existing_dir() {
//    let muxed_path = &muxed_path();
//    create_muxed_dir(muxed_path);
//    assert_eq!(muxed_path.as_str().unwrap(), muxed_dir().as_str().unwrap());
//    cleanup_dir(muxed_path);
//}
