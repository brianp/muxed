use libc::system;
use std::ffi::CString;

static TMUX_NAME: &'static str = "tmux";

fn call(command: String) {
    let system_call = CString::new(format!("{} {}", TMUX_NAME, command)).unwrap();
    unsafe { system(system_call.as_ptr()); };
}

pub fn open(session_name: String) -> () {
    call(format!("new -s {} -n {}", session_name, "hello"));
}

static SELECT_LAYOUT: &'static str = "select-layout";
pub fn select_layout(window: String, layout: String) -> () {
    call(format!("{} -t {} {}", SELECT_LAYOUT, window, layout));
}

pub fn new_window(session_name: String, window_name: String) -> () {
    call(format!("new-window -s {} -n {}", session_name, window_name));
}
