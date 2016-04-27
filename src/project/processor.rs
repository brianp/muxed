use command::Command;
use tmux;

pub fn main(commands: &Vec<Command>) -> () {
    let sess = "test".to_string();

    let (first_window, exec_commands) = commands.split_at(1);
    tmux::new_session(sess.clone(), first_window[0].value.clone());

    for c in exec_commands {
        tmux::new_window(sess.clone(), c.value.clone());
    };

    tmux::open(sess.clone());
}
