use common::error::CommonError;
use std::{fmt, io};

#[derive(Debug)]
pub enum NewError {
    Common(CommonError),
    Io(io::Error),
    Write(String),
    Template(String),
}

impl fmt::Display for NewError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NewError::Common(e) => write!(f, "{}", e),
            NewError::Io(e) => write!(f, "{}", e),
            NewError::Write(msg) => write!(f, "Template write error: {}", msg),
            NewError::Template(msg) => write!(f, "Template error: {}", msg),
        }
    }
}

impl std::error::Error for NewError {}

impl From<CommonError> for NewError {
    fn from(err: CommonError) -> NewError {
        NewError::Common(err)
    }
}

impl From<io::Error> for NewError {
    fn from(err: io::Error) -> NewError {
        NewError::Io(err)
    }
}
