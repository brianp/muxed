use libc::system;
use std::ffi::CString;

static TMUX_NAME: &'static str = "tmux";

fn call(command: String) {
    let system_call = CString::new(format!("{} {}", TMUX_NAME, command)).unwrap();
    unsafe { system(system_call.as_ptr()); };
}

pub fn open(session_name: String) {
    let open_command = format!("new -s {} -n {}", session_name, "hello");
    call(open_command);
}

static SELECT_LAYOUT: &'static str = "select-layout";
pub fn select_layout(window: String, layout: String) {
    let command = format!("{} -t {} {}", SELECT_LAYOUT, window, layout);
    call(command);
}

pub fn new_window(session_name: String, window_name: String) {
    let open_command = format!("new-window -s {} -n {}", session_name, window_name);
    call(open_command);
}
