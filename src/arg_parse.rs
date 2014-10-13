use getopts::{getopts,OptGroup,Matches};
#[cfg(test)] use help;

pub fn command(matches: &Matches) -> &str {
    matches.free[0].as_slice()
}

pub fn value(matches: &Matches) -> &str {
    matches.free[1].as_slice()
}

/// Accept the current arguments and match them using getopts.
/// # Errors
/// Will fail if arguments are not found.
pub fn matches(tail: &[String], opts: &[OptGroup]) -> Matches {
    return match getopts(tail, opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_string()) }
    }
}

/// Validate the free text passed to the application. If the free text can match
/// an application command return `true` otherwise `false`.
pub fn valid_command(matches: &Matches) -> bool {
    let command = if matches.free.len() == 2 {
        matches.free[0].as_slice()
    } else {
        return false
    };

    return match command {
        "new"  => true,
        "open" => true,
        _      => false
    }
}

#[test]
fn command_returns_new() {
    let matches = &matches([String::from_str("new"), String::from_str("muxed")], help::opts());
    assert_eq!(command(matches), "new");
}

#[test]
fn command_returns_edit() {
    let matches = &matches([String::from_str("open"), String::from_str("muxed")], help::opts());
    assert_eq!(command(matches), "open");
}

#[test]
fn value_returns_muxed() {
    let matches = &matches([String::from_str("new"), String::from_str("muxed")], help::opts());
    assert_eq!(value(matches), "muxed");
}

#[test]
#[should_fail]
fn matches_returns_failure_with_bad_opts() {
    matches([String::from_str("-m")], help::opts());
}

#[test]
fn valid_command_new_returns_true() {
    let matches = &matches([String::from_str("new"), String::from_str("muxed")], help::opts());
    assert_eq!(valid_command(matches), true);
}

#[test]
fn valid_command_open_returns_true() {
    let matches = &matches([String::from_str("open"), String::from_str("muxed")], help::opts());
    assert_eq!(valid_command(matches), true);
}

#[test]
fn valid_command_value_returns_false() {
    let matches = &matches([String::from_str("value"), String::from_str("muxed")], help::opts());
    assert_eq!(valid_command(matches), false);
}
