use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlayError {
    #[error("Invalid URL {0}")]
    InvalidURL(String),
    #[error("Songbird unregistered")]
    Unregistered,
    #[error("Input error {0}")]
    InputError(songbird::input::error::Error),
}
