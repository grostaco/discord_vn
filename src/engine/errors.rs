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
