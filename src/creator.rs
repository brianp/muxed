use std::io::{File,fs};
use std::path::posix::Path;
use std::os::{homedir};
use std::rand::random;

pub fn new<'r>(name: &str) {
    let home_unwrap = homedir().unwrap();
    let path = format!("{}/.muxed/{}", home_unwrap.display(), name);
    File::create(&Path::new(path));
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
