use common::error::CommonError;
use std::{fmt, io};

#[derive(Debug)]
pub enum ListError {
    Common(CommonError),
    Io(io::Error),
}

impl fmt::Display for ListError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ListError::Common(e) => write!(f, "{}", e),
            ListError::Io(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for ListError {}

impl From<CommonError> for ListError {
    fn from(err: CommonError) -> ListError {
        ListError::Common(err)
    }
}

impl From<io::Error> for ListError {
    fn from(err: io::Error) -> ListError {
        ListError::Io(err)
    }
}
