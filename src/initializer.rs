use getopts::{getopts};
use help;

fn verify_command(command: &str) -> bool {
    match command {
        "new"  => true,
        "open" => true,
        _      => false
    }
}

fn run_command(command: &str) {
    println!("{}", command);
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

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        help::print_usage(program.as_slice(), opts);
        return;
    };

    if !verify_command(input.as_slice()) {
        help::print_usage(program.as_slice(), opts);
        return;
    }

    run_command(input.as_slice());
}

#[test]
fn verify_command_new_returns_true() {
  assert_eq!(verify_command("new"), true);
}

#[test]
fn verify_command_open_returns_true() {
  assert_eq!(verify_command("open"), true);
}

#[test]
fn verify_command_value_returns_false() {
  assert_eq!(verify_command("value"), false);
}

