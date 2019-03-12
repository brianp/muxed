use std::path::Path;

pub trait Command {
    fn call<S>(&self) -> Vec<&str>;
}

pub struct Session {
    pub name: String,
    pub window_name: String,
    pub root_path: Path
}

impl Command for Session {
    fn call<S>(&self) -> Vec<&str> {
        vec!("new", "-d", "-s", &self.name, "-n", &self.window_name, "-c", &self.root_path.to_str().unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct Window {
    pub session_name: String,
    pub name: String,
    // pub path: Path
}

impl Command for Window {
    fn call<S>(&self) -> Vec<&str> {
        vec!("new-window", "-t", &self.session_name, "-n", &self.name)
    }
}

#[derive(Debug, Clone)]
pub struct Split {
    pub target: String,
}

impl Command for Split {
    fn call<S>(&self) -> Vec<&str> {
        vec!("split-window", "-t", &self.target)
    }
}
