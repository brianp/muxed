pub mod command;
pub mod project;
pub mod tmux;

use self::command::Commands;
use self::project::parser;
use self::tmux::config::Config;
use args::Args;

pub fn exec(args: Args) -> Result<(), String> {
    let muxed_dir = match args.flag_p {
        Some(ref x) => Some(x.as_str()),
        None => None,
    };

    let yaml = project::read(&args.arg_project, &muxed_dir).unwrap();
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

    for command in &commands {
        command.as_trait().call();
    }

    Ok(())
}
