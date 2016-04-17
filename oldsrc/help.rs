use getopts::{optflag,usage,OptGroup};
use std::io::{println,print};

pub fn opts() -> [OptGroup; 2]{
    return [
        optflag("v", "version", "display the version."),
        optflag("h", "help", "print this help menu.")
    ]
}

#[allow(dead_code)]
pub fn print_usage(opts: &[OptGroup; 2]) {
    let space = "    ";
    println("Usage: muxed <command> [options]");
    println("");
    println("Commands:");
    println(format!("{}new [name]          create a new project file.", space).as_slice());
    print(  format!("{}open [name]         open an existing project file.", space).as_slice());
    println(usage("", opts).as_slice());
}

#[test]
fn opts_returns_two_options() {
    assert_eq!(opts().len(), 2);
}
