use std::error::Error;
use std::fmt;
use std;
use openapi;

pub type Result<T> = std::result::Result<T, OpenApiError>;

#[derive(Debug)]
pub enum OpenApiError {
    Parse(String),
    Validation(String),
}

impl fmt::Display for OpenApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OpenApiError::Parse(ref err) => err.fmt(f),
            OpenApiError::Validation(ref string) => write!(f, "{}", string),

        }
    }
}

impl Error for OpenApiError {
    fn description(&self) -> &str {
        match *self {
            OpenApiError::Parse(ref string) => string,
            OpenApiError::Validation(ref string) => string,
        }
    }
}

//openapi::errors::Error
impl From<openapi::errors::Error> for OpenApiError {
    fn from(err: openapi::errors::Error) -> OpenApiError {
        OpenApiError::Parse(err.to_string())
    }
}

impl From<&'static str> for OpenApiError {
    fn from(err: &'static str) -> OpenApiError {
        OpenApiError::Validation(err.to_string())
    }
}

impl From<String> for OpenApiError {
    fn from(err: String) -> OpenApiError {
        OpenApiError::Validation(err)
    }
}
