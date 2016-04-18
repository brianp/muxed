use libc::system;
use std::ffi::CString;

static TMUX_NAME: &'static str = "tmux";
static NEW_SESSION: &'static str = "new -s";

pub fn open(session_name: String) {
    let system_call = CString::new(format!("{} {} {}", TMUX_NAME, NEW_SESSION, session_name)).unwrap();
    unsafe { system(system_call.as_ptr()); };
}
