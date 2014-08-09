use std::io::fs;
use std::path::posix::Path;
use std::os::homedir;
#[cfg(test)] use test_helper::random_name;

/// Create the muxed directory and return the path if creation is successful.
pub fn create_muxed_dir(name: &String) -> Path {
    let path = &Path::new(format!("{}/.{}", homedir_string(), name));
    match fs::mkdir(path, ::std::io::UserRWX) {
        Ok(()) => (),
        Err(_e) => println!("Failed to create project {}", path.filename()),
    }

    path.clone()
}

/// Return a boolean if the ~/.muxed/ dir exists.
pub fn muxed_dir_exists(name: &String) -> bool {
    let path = &Path::new(format!("{}/.{}", homedir_string(), name));
    path.exists()
}

/// Return the users current homedir as a string.
pub fn homedir_string() -> String {
    let home_unwrap = homedir().unwrap();
    format!("{}", home_unwrap.display())
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
