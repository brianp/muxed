use crate::interpreter::error::InterpreterError;
use crate::tmux::error::TmuxError;
use common::error::CommonError;
use std::path::PathBuf;
use std::{fmt, io};
use yaml_rust;

#[derive(Debug)]
pub enum LoadError {
    Io(io::Error),
    YamlParse(yaml_rust::ScanError),
    Read(String, PathBuf, io::Error),
    Common(CommonError),
    Tmux(TmuxError),
    Serialization(serde_saphyr::Error),
    Interpreter(InterpreterError),
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoadError::Io(e) => write!(f, "IO error: {}", e),
            LoadError::Interpreter(e) => write!(f, "Parse error: {}", e),
            LoadError::YamlParse(msg) => write!(f, "YAML Parse error: {}", msg),
            LoadError::Read(project_name, directory, e) => write!(
                f,
                "No project configuration file was found with the name `{}` in the directory `{}`. Received error: {}",
                project_name,
                directory.display(),
                e
            ),
            LoadError::Common(e) => write!(f, "{}", e),
            LoadError::Tmux(e) => write!(f, "{}", e),
            LoadError::Serialization(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for LoadError {}

impl From<CommonError> for LoadError {
    fn from(err: CommonError) -> LoadError {
        LoadError::Common(err)
    }
}

impl From<yaml_rust::ScanError> for LoadError {
    fn from(err: yaml_rust::ScanError) -> LoadError {
        LoadError::YamlParse(err)
    }
}

impl From<io::Error> for LoadError {
    fn from(err: io::Error) -> LoadError {
        LoadError::Io(err)
    }
}

impl From<TmuxError> for LoadError {
    fn from(err: TmuxError) -> LoadError {
        LoadError::Tmux(err)
    }
}

impl From<serde_saphyr::Error> for LoadError {
    fn from(err: serde_saphyr::Error) -> LoadError {
        LoadError::Serialization(err)
    }
}

impl From<InterpreterError> for LoadError {
    fn from(err: InterpreterError) -> LoadError {
        LoadError::Interpreter(err)
    }
}
