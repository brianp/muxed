use crate::bash::BashError;
use crate::fish::FishError;
use crate::zsh::ZshError;
use std::env::VarError;
use std::fmt;

#[derive(Debug)]
pub enum AutocompleteError {
    ShellNotSupported,
    Var(VarError),
    Bash(BashError),
    Zsh(ZshError),
    Fish(FishError),
}

impl fmt::Display for AutocompleteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AutocompleteError::ShellNotSupported => {
                write!(f, "Sorry, that shell is not supported at this time")
            }
            AutocompleteError::Var(err) => write!(f, "{}", err),
            AutocompleteError::Bash(err) => write!(f, "Bash error: {}", err),
            AutocompleteError::Zsh(err) => write!(f, "Zsh error: {}", err),
            AutocompleteError::Fish(err) => write!(f, "Fish error: {}", err),
        }
    }
}

impl std::error::Error for AutocompleteError {}

impl From<VarError> for AutocompleteError {
    fn from(err: VarError) -> Self {
        AutocompleteError::Var(err)
    }
}

impl From<BashError> for AutocompleteError {
    fn from(err: BashError) -> Self {
        AutocompleteError::Bash(err)
    }
}

impl From<ZshError> for AutocompleteError {
    fn from(err: ZshError) -> Self {
        AutocompleteError::Zsh(err)
    }
}

impl From<FishError> for AutocompleteError {
    fn from(err: FishError) -> Self {
        AutocompleteError::Fish(err)
    }
}
