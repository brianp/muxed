use getopts::{optflag,usage,OptGroup};
use std::io::{println,print};

pub fn opts() -> [OptGroup, .. 2]{
    return [
        optflag("v", "version", "display the version"),
        optflag("h", "help", "print this help menu")
    ]
}

pub fn print_usage(_program: &str, opts: &[OptGroup]) {
    let space = "    ";
    println("Usage: muxed <command> [options]");
    println("");
    println("Commands:");
    println(format!("{}new [name]          create a new project file", space).as_slice());
    print(format!("{}open [name]         open a new project file", space).as_slice());
    println(usage("", opts).as_slice());
}
