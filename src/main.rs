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
use clap::{Arg, App, AppSettings};
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
                      .setting(AppSettings::TrailingVarArg)
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
                           .short("p")
                           .multiple(false)
                           .value_name("PROJECT_DIR")
                           .takes_value(true)
                           .help("The directory your project config files live in. Defaults to ~/.muxed/"))
                      .arg(Arg::with_name("REST")
                          .multiple(true)
                          .hidden(true))
                      .get_matches();

    let project_name = matches.value_of("PROJECT_NAME").unwrap();
    let daemonize = matches.is_present("daemonize");
    let muxed_dir = matches.value_of("PROJECT_DIR");

    match project_name {
        "new" => {
            let mut cmd = process::Command::new("muxednew");
            if matches.is_present("REST") {
                let trail: Vec<&str> = matches.values_of("REST").unwrap().collect();
                trail.iter().fold(&mut cmd, |c, i| c.arg(i));
            };
            let result = cmd.output().unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

            println!("{}", String::from_utf8_lossy(&result.stdout));
            println!("{}", String::from_utf8_lossy(&result.stderr));
            return;
        }
        // Continue on
        _     => {}
    }

    let project_name = project_name.to_string();

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
