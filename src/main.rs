//! Muxed. A tmux project manager with no runtime dependencies.
extern crate libc;
extern crate yaml_rust;
extern crate docopt;
extern crate rustc_serialize;

#[cfg(test)] extern crate rand;

mod tmux;
mod command;
mod project;

use project::parser;
use project::processor;
use tmux::config::Config;
use command::Command;
use std::{process, env};
use std::process::exit;
use docopt::Docopt;

#[macro_export]
macro_rules! try_or_err (
    ($expr: expr) => ({
        match $expr {
            Ok(val) => val,
            Err(e) => {
                println!("Muxed ran in to a problem:");
                println!("{}", e);
                exit(1);
            }
        }
    })
);

static USAGE: &'static str = "
Usage:
    muxed [options] <project>
    muxed new [options] <project>
    muxed (-h | --help)
    muxed (-v | --version)

Flags:
    -d                  If you want to create a muxed session without connecting to it
    -h, --help          Prints help information
    -v, --version       Prints version information

Options:
    -p <project_dir>    The directory your project config files live in. Defaults to ~/.muxed/

Args:
    <project>           The name of your project to open

Subcommands:
    new                 The name of your project to create
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_d: bool,
    flag_v: bool,
    flag_p: Option<String>,
    arg_project: String,
    cmd_new: bool,
}

/// The main execution method.
/// Currently accepts a single option. The option represents a configuration
/// file in the same naming format. Given a project file name `projectName.yml`
/// in the `~/.muxed/` directory.
///
/// # Examples
///
/// ~/.muxed/projectName.yml
///
/// ```
/// root: ~/projects/muxed/
/// windows:
///     - cargo: "cargo build"
///     - vim: "vim ."
///     - git: ""
/// ```
///
/// You can run the command:
///
/// ```
/// $ ./muxed projectName
/// ```
pub fn main() {
    // First see if we have a subcommand. If we do we want to
    // skip decoding for docopt and passoff execution to the 
    // subcommand bin.
    let mut input: std::env::Args = env::args();

    if let Some(x) = input.nth(1) {
        match x.as_ref(){
            "new" => run_subcommand("muxednew", input),
            _     => {}
        }
    }

    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());

    if args.flag_v {
        println!("Muxed {}", env!("CARGO_PKG_VERSION"));
        exit(0);
    };

    let muxed_dir = match args.flag_p {
        Some(ref x) => Some(x.as_str()),
        None        => None
    };

    let yaml = try_or_err!(project::read(&args.arg_project, &muxed_dir));
    let project_name = &yaml[0]["name"].as_str().unwrap_or(&args.arg_project).to_string();

    let commands: Vec<Command>;
    match project::session_exists(project_name) {
        Some(c) => {
            commands = vec!(c);
        },
        None => {
            let config = Config::from_string(tmux::get_config());
            commands = try_or_err!(parser::call(&yaml, project_name, args.flag_d, config));
        }
    };

    processor::main(&commands)
}

pub fn run_subcommand(subc: &str, input: std::env::Args) {
    let mut cmd = process::Command::new(subc);
    let trail: Vec<String> = input.collect();
    cmd.args(trail.as_slice());

    let result = try_or_err!(cmd.output().map_err(|e| format!("It looks like {} might not be installed or we don't have access to it.\nWe received this system error while trying to call the subcommand `{}`: `{}`", subc, subc, e)));
    // Lets add an error code they can call on for more details. Why
    // isn't muxed new installed?
    println!("{}", String::from_utf8_lossy(&result.stdout));
    if let Some(c) = result.status.code() { exit(c); };
    exit(0);
}
