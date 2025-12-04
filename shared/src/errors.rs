use thiserror::Error;

#[derive(Debug, Error)]
pub enum SharedLibError {
    #[error("AES error: {0}")]
    AESError(String),
    #[error("UTF-8 error: {0}")]
    UTF8Error(String),
}

impl From<aes_gcm::Error> for SharedLibError {
    fn from(err: aes_gcm::Error) -> Self {
        Self::AESError(err.to_string())
    }
}

impl From<std::string::FromUtf8Error> for SharedLibError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::UTF8Error(err.to_string())
    }
}
