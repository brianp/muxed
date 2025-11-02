use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    NoDocFound,
    NoWindows,
    WindowNameRequired,
    WindowTargetRequired,
    KeysNotFound,
    FormatNotRecognized,
    BadCommandSplit,
    PanesConversion,
    HomeDirExpansion,
    ExpandPath,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::NoDocFound => write!(f, "No YAML document was found"),
            ParseError::NoWindows => write!(f, "No Windows have been defined"),
            ParseError::WindowNameRequired => write!(f, "Window should have a name"),
            ParseError::WindowTargetRequired => write!(f, "No target specified"),
            ParseError::KeysNotFound => write!(f, "Send keys not parseable"),
            ParseError::FormatNotRecognized => {
                write!(f, "Muxed config file formatting isn't recognized")
            }
            ParseError::BadCommandSplit => write!(f, "Commands failed to split"),
            ParseError::PanesConversion => write!(f, "Something is wrong with panes"),
            ParseError::HomeDirExpansion => write!(f, "Home dir could not be expanded"),
            ParseError::ExpandPath => write!(f, "Path could not be expanded"),
        }
    }
}

impl std::error::Error for ParseError {}
