use std::path::posix::Path;
#[cfg(not(test))] use std::os::homedir;

pub mod io;

static MUXED_NAME_STR: &'static str = "muxed";

/// Return the users current homedir as a string.
#[cfg(not(test))] fn homedir_string() -> String {
    match homedir() {
        Some(dir) => format!("{}", dir.display()), 
        None      => fail!("Impossible to get your home dir!")
    }
}

#[cfg(test)] fn homedir_string() -> String {
  String::from_str("/tmp")
}

pub fn path() -> Path {
    Path::new(format!("{}/.{}/", homedir_string(), &MUXED_NAME_STR.to_string()))
}

#[test]
pub fn path_returns_muxed_inside_homedir() {
    let path = format!("{}", path().display());
    let new  = format!("{}", Path::new("/tmp/.muxed").display());
    assert_eq!(path, new)
}
