#[derive(Debug)]
#[derive(Clone)]
pub struct Window {
  pub value: String
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Root {
  pub value: String,
  pub window: String
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Command {
    Window(Window),
    Root(Root)
}