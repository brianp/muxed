//! Muxed. A tmux project manager with no runtime dependencies.
extern crate common;
extern crate docopt;
extern crate edit;
extern crate load;
extern crate new;
extern crate snapshot;

use common::args::Args;
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

static DISALLOWED_PROJECT_NAMES: [&str; 2] = ["new", "edit"];

static USAGE: &str = "
Usage:
    muxed [options] <project>
    muxed edit [options] <project>
    muxed new [options] <project>
    muxed snapshot [options] <project>
    muxed (-h | --help)
    muxed (-v | --version)

Flags:
    -d                  If you want to create a muxed session without connecting to it
    -f                  Overwrite existing file if one exists
    --debug             Prints debug information while executing (project opening only)
    -h, --help          Prints help information
    -v, --version       Prints version information

Options:
    -p <project_dir>    The directory your project config files live in. Defaults to ~/.muxed/
    -t <session>        The name of the running TMUX session to codify

Args:
    <project>           The name of your project to open

Subcommands:
    edit <project>                   Edit an existing project file
    new <project>                    To create a new project file
    snapshot -t <session> <project>  Capture a running session and create a config file for it
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
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if args.flag_v {
        println!("Muxed {}", env!("CARGO_PKG_VERSION"));
        exit(0);
    };

    if args.cmd_new {
        try_or_err!(new::exec(args))
    } else if args.cmd_edit {
        try_or_err!(edit::exec(args))
    } else if args.cmd_snapshot {
        try_or_err!(snapshot::exec(args))
    } else if DISALLOWED_PROJECT_NAMES.contains(&args.arg_project.as_ref()) {
        println!(
            "Tried to call sub-command {} without a project. Please specify a project name.",
            args.arg_project
        );
        exit(1);
    } else {
        try_or_err!(load::exec(args))
    }
}
