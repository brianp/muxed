use std::io::File;
use std::os::{homedir};

pub fn new(name: &str) {
  let home = homedir().unwrap().display();
  let path = format!("{}/.muxed/{}", home, name);
  println!("{}", path);
  let mut file = File::create(&Path::new(path));
  file.write(b"This is a sample file");
}
