use std::env::VarError;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::{env, fmt, fs, io};

const BASH_COMPLETION: &str = r#"_muxed() {
    local cur prev opts projects
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    opts="list ls edit load new snapshot autocomplete"
    projects="$(muxed list -1 2>/dev/null)"

    # First argument after 'muxed' — offer commands + project names
    if [[ ${COMP_CWORD} -eq 1 ]]; then
        COMPREPLY=( $(compgen -W "${opts} ${projects}" -- "${cur}") )
        return 0
    fi

    # If the previous word is a command that expects a project name
    case "${prev}" in
        edit|load|snapshot)
            COMPREPLY=( $(compgen -W "${projects}" -- "${cur}") )
            return 0
            ;;
    esac
}
complete -F _muxed muxed
"#;

#[derive(Debug)]
pub enum BashError {
    Var(VarError),
    Io(io::Error),
}

impl fmt::Display for BashError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BashError::Var(err) => write!(f, "{}", err),
            BashError::Io(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for BashError {}

impl From<io::Error> for BashError {
    fn from(err: io::Error) -> Self {
        BashError::Io(err)
    }
}

impl From<VarError> for BashError {
    fn from(err: VarError) -> Self {
        BashError::Var(err)
    }
}

pub fn install() -> Result<(), BashError> {
    let home = env::var("HOME")?;
    let completion_dir = PathBuf::from(&home).join(".bash_completion.d");
    fs::create_dir_all(&completion_dir)?;

    let completion_file = completion_dir.join("muxed");
    fs::write(&completion_file, BASH_COMPLETION)?;

    let bashrc = PathBuf::from(&home).join(".bashrc");
    let source_line = format!("source {}\n", completion_file.display());

    if bashrc.exists() {
        let content = fs::read_to_string(&bashrc)?;
        if !content.contains(source_line.trim()) {
            let mut file = OpenOptions::new().append(true).open(&bashrc)?;
            writeln!(file, "\n# muxed completion")?;
            write!(file, "{}", source_line)?;
        }
    }

    println!("✓ Bash completion installed. Run: source ~/.bashrc");
    Ok(())
}
