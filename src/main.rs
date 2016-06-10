//! Muxed. A tmux project manager with no runtime dependencies.
extern crate clap;
extern crate libc;
extern crate yaml_rust;
#[cfg(test)] extern crate rand;

mod tmux;
mod command;
mod project;

use project::parser;
use project::processor;
use clap::{Arg, App, SubCommand, AppSettings};
use command::Command;
use std::process;

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
                          .version(env!("CARGO_PKG_VERSION"))
                          .author("Brian Pearce")
                          .about("Another TMUX project manager")
                          .setting(AppSettings::SubcommandsNegateReqs)
                          .arg(Arg::with_name("PROJECT_NAME")
                               .help("The name of your poject to open")
                               .index(1)
                               .required(true)
                               .takes_value(true))
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
                          .subcommand(SubCommand::with_name("new")
                                      .about("Create a new project file")
                                      .arg(Arg::with_name("NEW_PROJECT_NAME")
                                          .help("The new project/file name")
                                          .required(true))
                                      .arg(Arg::with_name("PROJECT_DIR")
                                          .short("-p")
                                          .multiple(false)
                                          .takes_value(true)
                                          .help("The directory your project config files live in. Defaults to ~/.muxed/")))
                          .get_matches();

    if let Some(matches) = matches.subcommand_matches("new") {
        if matches.is_present("NEW_PROJECT_NAME") {
            let project_name = matches.value_of("NEW_PROJECT_NAME").unwrap().to_string();

            let mut output = process::Command::new("muxednew").arg(project_name);

            if matches.is_present("PROJECT_DIR") {
                output.arg("-p")
                      .arg(matches.value_of("PROJECT_DIR").unwrap());
            }

            output = try_or_err!(output.output().map_err(|_| "Muxednew might not be installed." ));

            if !output.status.success() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            };
        };

        return
    };

    let project_name = matches.value_of("PROJECT_NAME").unwrap().to_string();
    let daemonize = matches.is_present("daemonize");
    let muxed_dir = matches.value_of("PROJECT_DIR");

    let commands: Vec<Command>;
    // This refactoring could make a good conference talk example
    match project::session_exists(&project_name) {
        Some(c) => {
            commands = vec!(c);
        },
        None => {
            let yaml = try_or_err!(project::read(&project_name, &muxed_dir));
            commands = try_or_err!(parser::main(&yaml, &project_name, daemonize));
        }
    };

    processor::main(&commands)
}
