//! Muxed. A tmux project manager with no runtime dependencies.
extern crate getopts;
extern crate libc;

use std::os;
use std::io::{println};

mod help;
mod arg_parse;
mod muxed_root;
mod creator {
  mod io { }
}

/// The main execution method.
/// Verify all the arguments and options passed are valid for the application.
fn main() {
    let args: Vec<String> = os::args();

    let opts = help::opts();
    let maybe_matches = arg_parse::matches(args.tail(), opts.clone());

    if maybe_matches.is_none() {
        help::print_usage(opts);
        return;
    }

    let matches = maybe_matches.unwrap();

    if matches.opt_present("v") {
        println(format!("{}", "Version: 0.0.1").as_slice());
        return;
    } else if matches.opt_present("h") || !arg_parse::valid_command(&matches) {
        help::print_usage(opts);
        return;
    }

    let fragments = &matches.free;
    let command   = arg_parse::command(fragments);
    let file_path = arg_parse::file_path(&muxed_root::path(), fragments);


    match command {
//        "new"  => creator::new(value),
        _      => help::print_usage(opts)
    }
}
