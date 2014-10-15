use std::path::posix::Path;
use std::os::homedir;

static MUXED_NAME_STR: &'static str = "muxed";

/// Return the users current homedir as a string.
fn homedir_string() -> String {
    let home_unwrap = homedir().unwrap();
    format!("{}", home_unwrap.display())
}

pub fn path() -> Path {
    Path::new(format!("{}/.{}/", homedir_string(), &MUXED_NAME_STR.to_string()))
}
