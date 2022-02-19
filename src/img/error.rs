use image::ImageError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoadImageError {
    #[error("Image error {0}")]
    ImageError(ImageError),
    #[error("IO Error {0}")]
    IoError(std::io::Error),
}
