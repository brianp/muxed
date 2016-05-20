//! Muxed. A tmux project manager with no runtime dependencies.
extern crate clap;
extern crate libc;
extern crate yaml_rust;
extern crate rand;

mod tmux;
mod command;
mod project;

use project::parser;
use project::processor;
use clap::{Arg, App};

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
    let matches = App::new("Muxed")
                          .version("0.2.0")
                          .author("Brian Pearce")
                          .about("Another TMUX window manager")
                          .arg(Arg::with_name("PROJECT_NAME")
                               .help("The name of your poject to open")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("daemonize")
                               .short("d")
                               .multiple(false)
                               .help("If you want to create a muxed session without connecting to it"))
                          .arg(Arg::with_name("PROJECT_DIR")
                               .short("-p")
                               .multiple(false)
                               .value_name("PROJECT_DIR")
                               .takes_value(true)
                               .help("The directory your project config files live in. Defaults to ~/.muxed/"))
                          //.subcommand(SubCommand::with_name("new")
                          //            .about("Create a new project file"))
                          //.subcommand(SubCommand::with_name("edit")
                          //            .about("Edit a project file"))
                          .get_matches();

    let input = matches.value_of("PROJECT_NAME").unwrap().to_string();
    let daemonize = matches.is_present("daemonize");
    let muxed_dir = matches.value_of("PROJECT_DIR");

    let yaml = try_or_err!(project::read(&input, &muxed_dir));
    let commands = try_or_err!(parser::main(&yaml, &input, daemonize));
    processor::main(&commands)
}
