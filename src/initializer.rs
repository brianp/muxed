use getopts::{getopts,OptGroup,Matches};
use help;
use creator;
use editor;


fn run_command(command: &str, value: &str) {
    match command {
        "new"  => creator::new(value),
        "open" => editor::new(value)
    }
}

#[test]
fn run_command_prints_value() {
  assert_eq!(run_command("value"), false);
}

pub fn init(args: Vec<String>) {
    let program = args[0].clone();
    let opts = help::opts();

    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_string()) }
    };

    if matches.opt_present("h") {
        help::print_usage(program.as_slice(), opts);
        return;
    }

    let input = if !matches.free.is_empty() && !matches.free.len() == 2 {
fn validate_command(matches: Matches) -> bool {
    let command = if matches.free.len() == 2 {
        matches.free[0].clone()
    } else {
        help::print_usage(program.as_slice(), opts);
        return;
    };

    if !verify_command(input.as_slice()) {
        help::print_usage(program.as_slice(), opts);
        return;
    return match command {
        "new"  => true,
        "open" => true,
        _      => false
    }
}

    run_command(input.as_slice(), matches.free[1].clone().as_slice());
#[test]
fn validate_command_new_returns_true() {
    let matches = get_matches([String::from_str("new muxed")], help::opts());
    assert_eq!(validate_command(matches), true);
}

#[test]
fn validate_command_open_returns_true() {
    let matches = get_matches([String::from_str("open muxed")], help::opts());
    assert_eq!(validate_command(matches), true);
}

#[test]
fn validate_command_value_returns_false() {
    let matches = get_matches([String::from_str("value")], help::opts());
    assert_eq!(validate_command(matches), false);
}

#[test]
#[should_fail]
fn get_matches_returns_failure_with_bad_opts() {
    get_matches([String::from_str("-m")], help::opts());
}

