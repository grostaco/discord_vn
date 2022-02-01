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

pub struct SyntaxError {
    pub file: String,
    pub line: usize,
    pub character: usize,
    pub why: &'static str,
}

from_error!(io::Error, ParseError, IoError);
from_error!(SyntaxError, ParseError, SyntaxError);

impl fmt::Debug for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{} {}",
            self.file, self.line, self.character, self.why
        )
    }
}