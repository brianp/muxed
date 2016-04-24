//! Muxed. A tmux project manager with no runtime dependencies.
extern crate getopts;
extern crate libc;
extern crate yaml_rust;

use std::env;

mod tmux;
mod project;

/// The main execution method.
/// Verify all the arguments and options passed are valid for the application.
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let input = args[1].clone();

    project::open(input.clone());
}
