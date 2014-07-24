extern crate getopts;
use std::os;

mod initializer;
mod help;
mod creator;
mod editor;

fn main() {
    let args: Vec<String> = os::args();
    initializer::init(args);
}
