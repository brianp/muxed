use command::Command;
use tmux;

pub fn main(commands: Vec<Command>) -> () {
    for c in commands.clone() {
        match c.clone() {
            Command::Session(c)  => tmux::new_session(c.name.clone(), c.window_name.clone()),
            Command::Window2(c)  => tmux::new_window(c.session_name.clone(), c.name.clone(), c.root),
            Command::Split(c)    => tmux::split_window(c.target.clone(), c.root),
            Command::Layout(c)   => tmux::layout(c.target.clone(), c.layout.clone()),
            Command::SendKeys(c) => tmux::send_keys(c.target.clone(), c.exec.clone()),
            Command::Attach(c)   => tmux::attach(c.name.clone()),
            _ => panic!("failed 2")
        }
    };
}
