use std::path::posix::Path;
use getopts::{getopts,OptGroup,Matches};
#[cfg(test)] use help;

pub fn command(fragments: &Vec<String>) -> &String {
    &fragments[0]
}

pub fn file_path(path: &Path, fragments: &Vec<String>) -> Path {
    Path::new(format!("{}/{}.toml", path.display(), fragments[1]))
}

/// Accept the current arguments and match them using getopts.
pub fn matches(tail: &[String], opts: &[OptGroup]) -> Option<Matches> {
    return match getopts(tail, opts) {
        Ok(m)  => { Some(m) }
        Err(_) => { None }
    }
}

/// Validate the free text passed to the application. If the free text can match
/// an application command return `true` otherwise `false`.
pub fn valid_command(matches: &Matches) -> bool {
    let command = match matches.free.len() {
        2 => matches.free[0].as_slice(),
        _ => return false
    };

    return match command {
        "new"  => true,
        "open" => true,
        _      => false
    }
}

#[test]
fn command_returns_new() {
    let matches = &matches([String::from_str("new"), String::from_str("muxed")], help::opts()).unwrap();
    assert_eq!(command(&matches.free), &String::from_str("new"));
}

#[test]
fn command_returns_edit() {
    let matches = &matches([String::from_str("open"), String::from_str("muxed")], help::opts()).unwrap();
    assert_eq!(command(&matches.free), &String::from_str("open"));
}

#[test]
fn file_path_returns_file() {
    let matches    = &matches([String::from_str("new"), String::from_str("muxed")], help::opts()).unwrap();
    let path       = &Path::new("/tmp/.muxed/");
    let muxed_file = format!("{}", Path::new("/tmp/.muxed/muxed.toml").display());
    assert_eq!(format!("{}", file_path(path, &matches.free).display()), muxed_file);
}

#[test]
fn file_path_returns_middle() {
    let matches    = &matches([String::from_str("new"), String::from_str("middle"), String::from_str("end")], help::opts()).unwrap();
    let path       = &Path::new("/tmp/.muxed/");
    let muxed_file = format!("{}", Path::new("/tmp/.muxed/middle.toml").display());
    assert_eq!(format!("{}", file_path(path, &matches.free).display()), muxed_file);
}

#[test]
fn matches_returns_failure_with_bad_opts() {
    assert!(matches([String::from_str("-m")], help::opts()).is_none());
}

#[test]
fn matches_returns_with_good_opts() {
    assert!(matches([String::from_str("-v")], help::opts()).is_some());
}

#[test]
fn valid_command_new_returns_true() {
    let matches = &matches([String::from_str("new"), String::from_str("muxed")], help::opts()).unwrap();
    assert_eq!(valid_command(matches), true);
}

#[test]
fn valid_command_open_returns_true() {
    let matches = &matches([String::from_str("open"), String::from_str("muxed")], help::opts()).unwrap();
    assert_eq!(valid_command(matches), true);
}

#[test]
fn valid_command_file_path_returns_false() {
    let matches = &matches([String::from_str("file_path"), String::from_str("muxed")], help::opts()).unwrap();
    assert_eq!(valid_command(matches), false);
}
