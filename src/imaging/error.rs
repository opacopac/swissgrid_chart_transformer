use image::ImageError;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum ImagingError {
    #[error(transparent)]
    IoError(#[from] ImageError),

    #[error("Invalid Argument: {0}")]
    InvalidArgumentError(String)
}
