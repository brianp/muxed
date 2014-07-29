use std::io::File;
use std::path::posix::Path;
use std::os::{homedir};

pub fn new<'r>(name: &str) {
    let home_unwrap = homedir().unwrap();
    let path = format!("{}/.muxed/{}", home_unwrap.display(), name);
    File::create(&Path::new(path));
}

#[test]
fn new_writes_file_to_muxed_dir() {
    let name = "test_project";
    let home_unwrap = homedir().unwrap();
    let path = &Path::new(format!("{}/.muxed/{}", home_unwrap.display(), name));
    new(name);
    assert!(path.exists());
}
