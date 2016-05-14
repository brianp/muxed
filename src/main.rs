//! Muxed. A tmux project manager with no runtime dependencies.
extern crate getopts;
extern crate libc;
extern crate yaml_rust;
extern crate rand;

use std::env;

mod tmux;
mod command;
mod project;

use project::parser;
use project::processor;

#[macro_export]
macro_rules! try_or_err (
    ($expr: expr) => ({
        match $expr {
            Ok(val) => val,
            Err(e) => {
              println!("Muxed ran in to a problem:");
              println!("{}", e);
              return
            }
        }
    })
);

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
    let args: Vec<String> = env::args().collect();
    //let program = args[0].clone();
    let input = &args[1];

    let yaml = try_or_err!(project::read(input));
    let commands = parser::main(&yaml, input);
    processor::main(&commands)
}
