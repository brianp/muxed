use project::parser::Command;
use tmux;

pub fn main(commands: &Vec<Command>) -> () {
    let sess = "test".to_string();

    let (first_window, exec_commands) = commands.split_at(1);
    tmux::new_session(sess.clone(), first_window[0].value.clone());

    for w in exec_commands {
        tmux::new_window(sess.clone(), w.value.clone());
    };

    tmux::open(sess.clone());
}
