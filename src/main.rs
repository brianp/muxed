//! Muxed. A tmux project manager with no runtime dependencies.

extern crate getopts;
extern crate libc;
use std::os;

mod initializer;
mod help;
mod root;
mod creator;
mod editor;
#[cfg(test)] mod test_helper;

/// The main execution method.
/// Uses getopts to fetch arguments and pass them to the initializer for
/// inspection.
fn main() {
    let args: Vec<String> = os::args();
    initializer::init(args);
}
