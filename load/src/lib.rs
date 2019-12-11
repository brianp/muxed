extern crate dirs;
extern crate libc;
extern crate yaml_rust;

extern crate common;

pub mod command;
pub mod project;
pub mod tmux;

use args::Args;
use command::Commands;
use common::project_paths::project_paths;
use common::{args, first_run};
use project::parser;
use tmux::config::Config;

pub fn exec(args: Args) -> Result<(), String> {
    let project_paths = project_paths(&args);

    let yaml = project::read(&args.arg_project, &project_paths).unwrap();
    let project_name = &yaml[0]["name"]
        .as_str()
        .unwrap_or(&args.arg_project)
        .to_string();

    let commands: Vec<Commands>;
    match project::session_exists(project_name) {
        Some(c) => {
            commands = vec![c];
        }
        None => {
            let config = Config::from_string(tmux::get_config());
            commands = parser::call(&yaml, project_name, args.flag_d, &config)
                .expect("Couldn't parse commands");
        }
    };

    if args.flag_debug {
        println!("{:?}", &commands);
    };

    for command in &commands {
        command
            .as_trait()
            .call(args.flag_debug)
            .map_err(|e| format!("Had a problem running commands for tmux {}", e))
            .unwrap();
    }

    Ok(())
}
