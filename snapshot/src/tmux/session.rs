use tmux::window::Window;

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub name: String,
    pub windows: Vec<Window>,
}

impl Session {
    pub fn new<S>(name: S, windows: Vec<Window>) -> Session
    where
        S: Into<String>,
    {
        Session {
            name: name.into(),
            windows,
        }
    }
}
