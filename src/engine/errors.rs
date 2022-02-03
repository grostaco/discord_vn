use std::{fmt, io};

macro_rules! from_error {
    ($from:ty,$to:ty,$variant:tt) => {
        impl From<$from> for $to {
            fn from(error: $from) -> Self {
                <$to>::$variant(error)
            }
        }
    };
}

#[derive(Debug)]
pub enum ParseError {
    SyntaxError(SyntaxError),
    IoError(io::Error),
}

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

#[derive(Debug)]
pub enum ScriptError {
    DirectiveError(DirectiveError),
    IoError(io::Error),
    SyntaxError(SyntaxError),
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
