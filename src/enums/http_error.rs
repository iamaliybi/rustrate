use std::fmt::{Debug, Display, Formatter};
use std::error::Error;

pub enum HttpError {
	ParseFailed,
	DecodeUrlFailed,
	UnsupportedMethod,
	UnsupportedVersion,
	MethodNotFound,
	UrlNotFound,
	VersionNotFound,
	ConnectionClosed,
}

impl Debug for HttpError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			HttpError::ParseFailed => write!(f, "ParseFailed"),
			HttpError::DecodeUrlFailed => write!(f, "DecodeUrlFailed"),
			HttpError::UnsupportedMethod => write!(f, "UnsupportedMethod"),
			HttpError::UnsupportedVersion => write!(f, "UnsupportedVersion"),
			HttpError::MethodNotFound => write!(f, "MethodNotFound"),
			HttpError::UrlNotFound => write!(f, "UrlNotFound"),
			HttpError::VersionNotFound => write!(f, "HttpVersionNotFound"),
			HttpError::ConnectionClosed => write!(f, "ConnectionClosed"),
		}
	}
}

impl Display for HttpError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			HttpError::ParseFailed => write!(f, "ParseFailed"),
			HttpError::DecodeUrlFailed => write!(f, "DecodeUrlFailed"),
			HttpError::UnsupportedMethod => write!(f, "UnsupportedMethod"),
			HttpError::UnsupportedVersion => write!(f, "UnsupportedVersion"),
			HttpError::MethodNotFound => write!(f, "MethodNotFound"),
			HttpError::UrlNotFound => write!(f, "UrlNotFound"),
			HttpError::VersionNotFound => write!(f, "HttpVersionNotFound"),
			HttpError::ConnectionClosed => write!(f, "ConnectionClosed"),
		}
	}
}

impl Error for HttpError {}