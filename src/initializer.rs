use getopts::{getopts,OptGroup,Matches};
use help;
use creator;
use editor;

pub fn init(args: Vec<String>) {
    let program = args[0].clone();
    let opts = help::opts();
    let matches = &get_matches(args.tail(), opts.clone());

    if matches.opt_present("h") || !validate_command(matches) {
        help::print_usage(program.as_slice(), opts);
        return;
    }

    run_command(matches);
}

fn get_matches(tail: &[String], opts: &[OptGroup]) -> Matches {
    return match getopts(tail, opts) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_string()) }
    }
}

fn validate_command(matches: &Matches) -> bool {
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

fn run_command(matches: &Matches) {
    let command = matches.free[0].as_slice();
    let value = matches.free[1].as_slice();

    match command {
        "new"  => creator::new(value),
        "open" => editor::new(value),
        _      => fail!()
    }
}

#[test]
fn validate_command_new_returns_true() {
    let matches = &get_matches([String::from_str("new"), String::from_str("muxed")], help::opts());
    assert_eq!(validate_command(matches), true);
}

#[test]
fn validate_command_open_returns_true() {
    let matches = &get_matches([String::from_str("open"), String::from_str("muxed")], help::opts());
    assert_eq!(validate_command(matches), true);
}

#[test]
fn validate_command_value_returns_false() {
    let matches = &get_matches([String::from_str("value"), String::from_str("muxed")], help::opts());
    assert_eq!(validate_command(matches), false);
}

#[test]
#[should_fail]
fn get_matches_returns_failure_with_bad_opts() {
    get_matches([String::from_str("-m")], help::opts());
}

