//! Muxed. A tmux project manager with no runtime dependencies.

extern crate getopts;
extern crate libc;
use std::os;

mod initializer;
mod help;
mod creator;
mod editor;

/// The main execution method.
/// Uses getopts to fetch arguments and pass them to the initializer for
/// inspection.
fn main() {
    let args: Vec<String> = os::args();
    initializer::init(args);
}
