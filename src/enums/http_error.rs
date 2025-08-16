use std::fmt::{Debug, Display, Formatter};
use std::error::Error;

pub enum HttpError {
	DecodeUrlFailed,
	UnsupportedMethod,
	UnsupportedVersion,
	MethodNotFound,
	UrlNotFound,
	VersionNotFound,
	ConnectionClosed,
	HeadersTooLarge,
	BodyTooLarge,
	BodyNotFound,
	RequestLineNotFound,
	RequestTimeout,
	NotImplemented,
	TooManyRequests,
}

impl Debug for HttpError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			HttpError::DecodeUrlFailed => write!(f, "DecodeUrlFailed"),
			HttpError::UnsupportedMethod => write!(f, "UnsupportedMethod"),
			HttpError::UnsupportedVersion => write!(f, "UnsupportedVersion"),
			HttpError::MethodNotFound => write!(f, "MethodNotFound"),
			HttpError::UrlNotFound => write!(f, "UrlNotFound"),
			HttpError::VersionNotFound => write!(f, "HttpVersionNotFound"),
			HttpError::ConnectionClosed => write!(f, "ConnectionClosed"),
			HttpError::HeadersTooLarge => write!(f, "HeadersTooLarge"),
			HttpError::BodyTooLarge => write!(f, "BodyTooLarge"),
			HttpError::BodyNotFound => write!(f, "BodyNotFound"),
			HttpError::RequestLineNotFound => write!(f, "RequestLineNotFound"),
			HttpError::RequestTimeout => write!(f, "RequestTimeout"),
			HttpError::NotImplemented => write!(f, "NotImplemented"),
			HttpError::TooManyRequests => write!(f, "TooManyRequests"),
		}
	}
}

impl Display for HttpError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			HttpError::DecodeUrlFailed => write!(f, "DecodeUrlFailed"),
			HttpError::UnsupportedMethod => write!(f, "UnsupportedMethod"),
			HttpError::UnsupportedVersion => write!(f, "UnsupportedVersion"),
			HttpError::MethodNotFound => write!(f, "MethodNotFound"),
			HttpError::UrlNotFound => write!(f, "UrlNotFound"),
			HttpError::VersionNotFound => write!(f, "HttpVersionNotFound"),
			HttpError::ConnectionClosed => write!(f, "ConnectionClosed"),
			HttpError::HeadersTooLarge => write!(f, "HeadersTooLarge"),
			HttpError::BodyTooLarge => write!(f, "BodyTooLarge"),
			HttpError::BodyNotFound => write!(f, "BodyNotFound"),
			HttpError::RequestLineNotFound => write!(f, "RequestLineNotFound"),
			HttpError::RequestTimeout => write!(f, "RequestTimeout"),
			HttpError::NotImplemented => write!(f, "NotImplemented"),
			HttpError::TooManyRequests => write!(f, "TooManyRequests"),
		}
	}
}

impl Error for HttpError {}