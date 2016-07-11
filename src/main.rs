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
use tmux::config::Config;
use clap::{Arg, App, AppSettings};
use command::Command;
use std::{process, env};
use std::process::exit;

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
    let mut app = App::new("Muxed")
                      .version(env!("CARGO_PKG_VERSION"))
                      .author("Brian Pearce")
                      .about("Another TMUX project manager")
                      .setting(AppSettings::TrailingVarArg)
                      .usage("muxed [FLAGS] [OPTIONS] <PROJECT_NAME>")
                      .arg(Arg::with_name("PROJECT_NAME")
                           .help("The name of your poject to open")
                           .index(1)
                           .multiple(false)
                           .takes_value(true))
                      .arg(Arg::with_name("daemonize")
                           .short("d")
                           .multiple(false)
                           .help("If you want to create a muxed session without connecting to it"))
                      .arg(Arg::with_name("help")
                           .short("h")
                           .long("help")
                           .help("Prints help information")
                           .takes_value(false))
                      .arg(Arg::with_name("PROJECT_DIR")
                           .short("p")
                           .multiple(false)
                           .value_name("PROJECT_DIR")
                           .takes_value(true)
                           .help("The directory your project config files live in. Defaults to ~/.muxed/"))
                      .arg(Arg::with_name("REST")
                          .multiple(true)
                          .hidden(true));

    let matches = &app.get_matches_from_safe_borrow(env::args()).unwrap();

    // We check for help twice. We don't want to short circuit early incase help
    // is supposed to get passed to the subcommand.
    let project_name: &str;
    if matches.value_of("PROJECT_NAME").is_some() {
        project_name = matches.value_of("PROJECT_NAME").unwrap();
    } else if matches.is_present("help") {
        let _ = &app.print_help();
        println!("\n");
        println!("SUBCOMMANDS:");
        println!("    new    The name of your poject to create\n");
        exit(0);
    } else {
        println!("No project name specified.");
        println!("error: The following required arguments were not provided:
<PROJECT_NAME>

USAGE:
muxed <PROJECT_NAME>");
        exit(1);
    };

    match project_name {
        "new" => {
            let mut cmd = process::Command::new("muxednew");
            if matches.is_present("REST") {
                let trail: Vec<&str> = matches.values_of("REST").unwrap().collect();
                trail.iter().fold(&mut cmd, |c, i| c.arg(i));
            };

            if matches.is_present("help") { cmd.arg("--help"); };

            let result = try_or_err!(cmd.output().map_err(|e| format!("It looks like muxednew might not be installed or we don't have access to it.\nWe received this system error while trying to call the subcommand `new`: `{}`", e)));
            // Lets add an error code they can call on for more details. Why
            // isn't muxed new installed?
            println!("{}", String::from_utf8_lossy(&result.stdout));
            println!("{}", String::from_utf8_lossy(&result.stderr));
            if let Some(c) = result.status.code() { exit(c); };
        }
        // No SubCommands found continue on.
        _     => {}
    }

    if matches.is_present("help") {
        let _ = &app.print_help();
        println!("\n");
        println!("SUBCOMMANDS:");
        println!("    new    The name of your poject to create\n");
        exit(0);
    };

    let project_name = project_name.to_string();
    let daemonize = matches.is_present("daemonize");
    let muxed_dir = matches.value_of("PROJECT_DIR");

    let commands: Vec<Command>;
    match project::session_exists(&project_name) {
        Some(c) => {
            commands = vec!(c);
        },
        None => {
            let config = Config::from_string(tmux::get_config());
            let yaml = try_or_err!(project::read(&project_name, &muxed_dir));
            commands = try_or_err!(parser::call(&yaml, &project_name, daemonize, config));
        }
    };

    processor::main(&commands)
}
