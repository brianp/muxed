use std::fmt;

#[derive(Debug)]
pub enum CommonError {
    ProjectPaths(String),
    FirstRunError(String),
}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommonError::ProjectPaths(msg) => write!(f, "ProjectPaths error: {}", msg),
            CommonError::FirstRunError(msg) => write!(f, "FirstRun error: {}", msg),
        }
    }
}

impl std::error::Error for CommonError {}
