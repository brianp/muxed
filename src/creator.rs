use std::io::{File,fs};
use std::path::posix::Path;
use std::os::{homedir};
use std::rand::random;

pub fn new<'r>(name: &str) {
    let home_unwrap = homedir().unwrap();
    let path = format!("{}/.muxed/{}", home_unwrap.display(), name);
    File::create(&Path::new(path));
}

fn create_muxed_dir(name: &String) -> Path {
    let home_unwrap = homedir().unwrap();
    let path = &Path::new(format!("{}/.{}", home_unwrap.display(), name));
    fs::mkdir(path, ::std::io::UserRWX);
    path.clone()
}
#[test]
fn creates_muxed_dir() {
    let name        = format!("test_project_{}", random::<f64>());
    let dir         = format!("test_dir_{}", random::<f64>());
    let home_unwrap = homedir().unwrap();
    let muxed_path  = &Path::new(format!("{}/.{}/", home_unwrap.display(), dir.as_slice()));
    create_muxed_dir(&dir);
    assert!(muxed_path.exists());
    fs::rmdir_recursive(muxed_path);
}

#[test]
fn new_writes_file_to_muxed_dir() {
    let name = format!("test_project_{}", random::<f64>());
    let home_unwrap = homedir().unwrap();
    let path = &Path::new(format!("{}/.muxed/{}", home_unwrap.display(), name));
    new(name);
    new(name.as_slice());
    assert!(path.exists());
    fs::unlink(path);
}
