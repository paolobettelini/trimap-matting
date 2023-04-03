use image::error::ImageError;
use opencv::Error as OpencvError;

#[derive(Debug)]
pub struct MessageError {
    pub message: String,
}

pub type MessageResult<T> = Result<T, MessageError>;

impl MessageError {
    pub fn new(message: String) -> Self {
        MessageError { message }
    }
}

impl From<String> for MessageError {
    fn from(message: String) -> Self {
        MessageError::new(message)
    }
}

impl From<&str> for MessageError {
    fn from(message: &str) -> Self {
        MessageError::new(message.to_string())
    }
}

impl From<OpencvError> for MessageError {
    fn from(error: OpencvError) -> Self {
        MessageError::new(error.message)
    }
}

impl From<ImageError> for MessageError {
    fn from(error: ImageError) -> Self {
        MessageError::new(error.to_string())
    }
}