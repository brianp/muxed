//! Muxed. A tmux project manager with no runtime dependencies.
#![feature(macro_rules)]

extern crate getopts;
extern crate libc;

use std::os;
use std::io::{println};

#[macro_export]
macro_rules! try_or_err (
    ($expr: expr, $err: expr) => ({
        match $expr {
            Ok(val) => val,
            Err(_e) => println!($err)
        }
    })
)

mod help;
mod arg_parse;
mod root;
mod project;
mod test_helper;

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
    let file_path = arg_parse::file_path(&root::path(), fragments);

    match command {
        "new"  => project::main(file_path),
        _      => help::print_usage(opts)
    }
}
