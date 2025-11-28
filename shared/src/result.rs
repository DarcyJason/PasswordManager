use crate::errors::SharedLibError;

pub type SharedLibResult<T> = Result<T, SharedLibError>;
