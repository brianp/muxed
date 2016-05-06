#[derive(Debug)]
#[derive(Clone)]
pub struct Session {
    pub name: String,
    pub window_name: String,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Window {
    pub session_name: String,
    pub name: String,
    pub root: Option<String>
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Split {
    pub target: String,
    pub root: Option<String>
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Layout {
    pub target: String,
    pub layout: String,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct SendKeys {
    pub target: String,
    pub exec: String
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Attach {
    pub name: String
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Command {
    Session(Session),
    Window(Window),
    Split(Split),
    Layout(Layout),
    SendKeys(SendKeys),
    Attach(Attach)
}
