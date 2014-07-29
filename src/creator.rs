use std::io::File;
use std::os::{homedir};

pub fn new<'r>(name: &str) {
  let home_unwrap = homedir().unwrap();
  let path = format!("{}/.muxed/{}", home_unwrap.display(), name);
  File::create(&Path::new(path));
}
