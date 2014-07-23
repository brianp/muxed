extern crate getopts;
use std::os;

mod initializer;
mod help;

fn main() {
    let args: Vec<String> = os::args();
    initializer::init(args);
}
