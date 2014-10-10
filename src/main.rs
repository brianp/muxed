//! Muxed. A tmux project manager with no runtime dependencies.
extern crate getopts;
extern crate libc;

use std::os;
use std::io::{println};
use getopts::{Matches};
use initializer::{get_matches,validate_command};
use creator::{new};

mod initializer;
mod help;
mod creator;

/// The main execution method.
/// Uses getopts to fetch arguments and pass them to the initializer for
/// inspection.
/// The init method accepts a `Vec<String>` of arguments. If an argument or
/// command does not match valid input print the help screen.
fn main() {
    let args: Vec<String> = os::args();

    let program = args[0].clone();
    let opts = help::opts();
    let matches = &get_matches(args.tail(), opts.clone());

    if matches.opt_present("v") {
        println(format!("{}", "Versions: 0.0.0").as_slice());
        return;
    }

    if matches.opt_present("h") || !validate_command(matches) {
        help::print_usage(program.as_slice(), opts);
        return;
    }

    run_command(matches);
}

/// Once the free text has been validated use the command to execute the
/// operation.
fn run_command(matches: &Matches, program: Program, opts: GetOpts) {
    let command = matches.free[0].as_slice();
    let value   = matches.free[1].as_slice();

    match command {
//        "new"  => creator::new(value),
        _      => help::print_usage(program.as_slice(), opts)
    }
}
