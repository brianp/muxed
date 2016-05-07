use command::Command;
use tmux;

pub fn main(commands: &Vec<Command>) -> () {
    for c in commands {
        match c {
            &Command::Session(ref c)    => tmux::new_session(&c.name, &c.tmp_window_name),
            &Command::Window(ref c)     => tmux::new_window(&c.session_name, &c.name, &c.root),
            &Command::Split(ref c)      => tmux::split_window(&c.target, &c.root),
            &Command::Layout(ref c)     => tmux::layout(&c.target, &c.layout),
            &Command::SendKeys(ref c)   => tmux::send_keys(&c.target, &c.exec),
            &Command::Attach(ref c)     => tmux::attach(&c.name),
            &Command::KillWindow(ref c) => tmux::kill_window(&c.name)
        }
    };
}
