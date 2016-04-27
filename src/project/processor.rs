use command::{Command, Window};
use tmux;

pub fn main(commands: Vec<Command>) -> () {
    let sess = "test".to_string();

    let first_window: Vec<Window> = commands.iter().flat_map(|c| {
        match c.clone() {
            Command::Window(w) => Some(w),
            _                  => None
        }
    }).collect();

    let (_, exec_commands) = commands.split_at(1);
    tmux::new_session(sess.clone(), first_window[0].value.clone());

    for c in exec_commands.clone() {
        match c.clone() {
            Command::Window(w) => tmux::new_window(sess.clone(), w.value.clone(), w.root),
            _ => panic!("nope")
        }
    };

    tmux::open(sess.clone());
}
