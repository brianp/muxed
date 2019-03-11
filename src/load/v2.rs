use std::path::Path;

pub trait Command {
    fn call<S>(&self) -> Vec<S>
    where
      S: Into<String>;
}

pub struct Session {
    pub name: String,
    pub window_name: String,
    pub root_path: Path
}

impl Command for Session {
    fn call<S>(&self) -> Vec<S>
    where
      S: Into<String>,
    {
        vec!("new", "-d", "-s", &self.name, "-n", &self.window_name, "-c", &self.root_path.to_str().unwrap())
    }
}
