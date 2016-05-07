use libc::system;
use std::ffi::CString;

static TMUX_NAME: &'static str = "tmux";

fn call(command: String) -> () {
    let line = format!("{} {}", TMUX_NAME, command);
    let system_call = CString::new(line.clone()).unwrap();
    println!("{}", line.clone());
    unsafe { system(system_call.as_ptr()); };
}

pub fn attach(session_name: &String) -> () {
    call(format!("attach -t {}", session_name));
}

pub fn new_session(session_name: &String, tmp_name: &String) -> () {
    call(format!("new -d -s {} -n {}", session_name, tmp_name));
}

pub fn split_window(target: &String, root: &Option<String>) -> () {
    if root.is_some() {
        call(format!("split-window -t {} -c {}", target, root.clone().unwrap()));
    } else {
        call(format!("split-window -t {}", target));
    }
}

pub fn new_window(session_name: &String, window_name: &String, root: &Option<String>) -> () {
    if root.is_some() {
        call(format!("new-window -t {} -n {} -c {}", session_name, window_name, root.clone().unwrap()));
    } else {
        call(format!("new-window -t {} -n {}", session_name, window_name));
    }
}

pub fn layout(target: &String, layout: &String) -> () {
    call(format!("select-layout -t {} {}", target, layout));
}

pub fn send_keys(target: &String, exec: &String) -> () {
    call(format!("send-keys -t {} '{}' KPEnter", target, exec));
}

pub fn kill_window(target: &String) -> () {
    call(format!("kill-window -t {}", target));
}
