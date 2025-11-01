use crate::tmux::error::TmuxError;
use common::error::CommonError;
use std::{fmt, io};
use yaml_rust;

#[derive(Debug)]
pub enum LoadError {
    Io(io::Error),
    Parse(String),
    YamlParse(yaml_rust::ScanError),
    Read(String),
    Common(CommonError),
    TmuxError(TmuxError),
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoadError::Io(e) => write!(f, "IO error: {}", e),
            LoadError::Parse(msg) => write!(f, "Parse error: {}", msg),
            LoadError::YamlParse(msg) => write!(f, "YAML Parse error: {}", msg),
            LoadError::Read(msg) => write!(f, "Read error: {}", msg),
            LoadError::Common(e) => write!(f, "{}", e),
            LoadError::TmuxError(e) => write!(f, "{}", e),
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
        LoadError::TmuxError(err)
    }
}
