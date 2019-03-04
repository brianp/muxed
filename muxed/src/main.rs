//! Muxed. A tmux project manager with no runtime dependencies.
extern crate dirs;
extern crate docopt;
extern crate libc;
extern crate rustc_serialize;
extern crate yaml_rust;

#[cfg(test)]
extern crate rand;

mod args;
mod load;
mod new;
mod project;
mod tmux;

use args::Args;
use docopt::Docopt;
use std::env;
use std::process::exit;

#[macro_export]
macro_rules! try_or_err (
    ($expr: expr) => ({
        match $expr {
            Ok(val) => val,
            Err(e) => {
                println!("Muxed ran in to a problem: {}", e);
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
    new <project>       The name of your project to create
";

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
    let mut input: std::env::Args = env::args();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.flag_v {
        println!("Muxed {}", env!("CARGO_PKG_VERSION"));
        exit(0);
    };

    if let Some(x) = input.nth(1) {
        match x.as_ref() {
            "new" => try_or_err!(new::exec(args)),
            // "snapshot" => try_or_err!(snapshot::exec(args)),
            _ => try_or_err!(load::exec(args)),
        }
    }
}
