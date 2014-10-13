//! Muxed. A tmux project manager with no runtime dependencies.
extern crate getopts;
extern crate libc;

use std::os;
use std::io::{println};

mod help;
mod arg_parse;
mod creator {
  mod io { }
}

/// The main execution method.
/// Uses getopts to fetch arguments and pass them to the initializer for
/// inspection.
/// The init method accepts a `Vec<String>` of arguments. If an argument or
/// command does not match valid input print the help screen.
fn main() {
    let args: Vec<String> = os::args();

    let opts = help::opts();
    let matches = &arg_parse::matches(args.tail(), opts.clone());

    if matches.opt_present("v") {
        println(format!("{}", "Version: 0.0.0").as_slice());
        return;
    } else if matches.opt_present("h") || !arg_parse::valid_command(matches) {
        help::print_usage(opts);
        return;
    }

    let command = arg_parse::command(matches);
    let _value  = arg_parse::value(matches);

    match command {
//        "new"  => creator::new(value),
        _      => help::print_usage(opts)
    }
}
