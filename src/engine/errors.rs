use std::fmt::Debug;

use thiserror::Error;

#[derive(Debug)]
pub struct SyntaxError {
    pub file: String,
    pub line: usize,
    pub character: usize,
    pub why: String,
}

#[derive(Debug)]
pub struct DirectiveError {
    pub why: &'static str,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unknown directive {0}")]
    UnknownDirective(String),
    #[error("Directive {0} cannot handle argument {1}")]
    DirectiveError(&'static str, String),
    #[error("Cannot open file {0}")]
    NoFileExists(String),
    #[error("Error on file {0} line {1} character {2}: {3}")]
    SyntaxError(String, usize, usize, String),
}

/*
macro_rules! from_error {
    ($from:ty,$to:ty,$variant:tt) => {
        impl From<$from> for $to {
            fn from(error: $from) -> Self {
                <$to>::$variant(error)
            }
        }
    };
}


from_error!(io::Error, ParseError, IoError);
from_error!(SyntaxError, ParseError, SyntaxError);
from_error!(DirectiveError, ScriptError, DirectiveError);
from_error!(io::Error, ScriptError, IoError);
from_error!(SyntaxError, ScriptError, SyntaxError);

impl fmt::Display for ScriptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DirectiveError(e) => write!(f, "{}", e),
            Self::SyntaxError(syn) => write!(f, "{}", syn),
            Self::IoError(io) => write!(f, "{}", io),
        }
    }
}

impl fmt::Display for DirectiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.why)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SyntaxError(syn) => write!(f, "{}", syn),
            Self::IoError(io) => write!(f, "{}", io),
        }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error on file {} line {} character {}: {}",
            self.file, self.line, self.character, self.why
        )
    }
}
*/
