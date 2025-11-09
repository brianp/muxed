use std::env::VarError;
use std::path::PathBuf;
use std::{env, fmt, fs, io};

const FISH_COMPLETION: &str = r#"function __fish_muxed_projects
    muxed list -1 2>/dev/null
end

function __fish_muxed_needs_command
    set cmd (commandline -opc)
    test (count $cmd) -eq 1
end

function __fish_muxed_needs_project
    set cmd (commandline -opc)
    set sub (string split ' ' -- $cmd)[2]
    contains -- $sub edit load snapshot
end

# Subcommands
complete -c muxed -n '__fish_muxed_needs_command' -a "list ls edit load new snapshot autocomplete"

# Project completions for commands that expect a project
complete -c muxed -n '__fish_muxed_needs_project' -a '(__fish_muxed_projects)'
"#;

#[derive(Debug)]
pub enum FishError {
    Var(VarError),
    Io(io::Error),
}

impl fmt::Display for FishError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FishError::Var(err) => write!(f, "{}", err),
            FishError::Io(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for FishError {}

impl From<io::Error> for FishError {
    fn from(err: io::Error) -> Self {
        FishError::Io(err)
    }
}

impl From<VarError> for FishError {
    fn from(err: VarError) -> Self {
        FishError::Var(err)
    }
}

pub fn install() -> Result<(), FishError> {
    let home = env::var("HOME")?;
    let completion_dir = PathBuf::from(&home)
        .join(".config")
        .join("fish")
        .join("completions");
    fs::create_dir_all(&completion_dir)?;

    fs::write(completion_dir.join("muxed.fish"), FISH_COMPLETION)?;

    println!("✓ Fish completion installed. Start new session to use.");
    Ok(())
}
