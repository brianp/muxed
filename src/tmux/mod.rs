use libc::system;
use std::ffi::CString;

static TMUX_NAME: &'static str = "tmux";

fn call(command: String) -> () {
    let line = format!("{} {}", TMUX_NAME, command);
    let system_call = CString::new(line.clone()).unwrap();
    //println!("{}", line.clone());
    unsafe { system(system_call.as_ptr()); };
}

pub fn open(session_name: String) -> () {
    call(format!("attach -t {}", session_name));
}

pub fn new_session(session_name: String, first_window: String) -> () {
    call(format!("new -d -s {} -n {}", session_name, first_window));
}

static SELECT_LAYOUT: &'static str = "select-layout";
pub fn select_layout(window: String, layout: String) -> () {
    call(format!("{} -t {} {}", SELECT_LAYOUT, window, layout));
}

pub fn new_window(session_name: String, window_name: String) -> () {
    call(format!("new-window -t {} -n {}", session_name, window_name));
}
