use chrono::ParseError;
use core::fmt;

#[derive(Debug)]
pub struct GenericError(pub String, pub String);

impl std::error::Error for GenericError {}

impl fmt::Display for GenericError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{} (because: {})", self.0, self.1)
	}
}

pub fn from_reqwest(message: &str, inner: reqwest::Error) -> GenericError {
	GenericError(message.to_string(), inner.to_string())
}

pub fn from_parser(message: &str, inner: ParseError) -> GenericError {
	GenericError(message.to_string(), inner.to_string())
}

pub fn generic_error(message: &str) -> GenericError {
	GenericError(message.to_string(), "".to_string())
}
