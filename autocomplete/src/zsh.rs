use std::env::VarError;
use std::path::PathBuf;
use std::{env, fmt, fs, io};

const ZSH_COMPLETION: &str = r#"#compdef muxed

_muxed() {
    local projectdir=~/.muxed
    local -a commands
    local -a projects
    commands=(list ls edit load new snapshot autocomplete)

    # Guaranteed stripping via for-loop just to be safe with all shells
    projects=("${(@f)$(muxed list -1 2>/dev/null)}")

    if (( CURRENT == 2 )); then
        # Merge commands and projects for the first positional argument
        compadd -- $commands $projects
    elif (( CURRENT == 3 )); then
        # When completing after a command that expects a project name
        if [[ "$words[2]" == (edit|load|snapshot) ]]; then
            compadd -- $projects
            return
        fi
    fi
}

compdef _muxed muxed
"#;

#[derive(Debug)]
pub enum ZshError {
    Var(VarError),
    Io(io::Error),
}

impl fmt::Display for ZshError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ZshError::Var(err) => write!(f, "{}", err),
            ZshError::Io(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for ZshError {}

impl From<io::Error> for ZshError {
    fn from(err: io::Error) -> Self {
        ZshError::Io(err)
    }
}

impl From<VarError> for ZshError {
    fn from(err: VarError) -> Self {
        ZshError::Var(err)
    }
}

pub fn install() -> Result<(), ZshError> {
    let home = env::var("HOME")?;
    let completion_dir = PathBuf::from(&home).join(".zsh").join("completions");
    fs::create_dir_all(&completion_dir)?;

    fs::write(completion_dir.join("_muxed"), ZSH_COMPLETION)?;

    println!("✓ Zsh completion installed. Run: exec zsh");

    Ok(())
}
