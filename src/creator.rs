use std::io::File;

pub fn new(name: &str) {
  let path = format!("~/.muxed/{}", name);
  println!("{}", path);
  let mut file = File::create(&Path::new(path));
  file.write(b"This is a sample file");
}
