pub enum HttpVersion {
	Http10,
	Http11,
	Http20,
	Http30,
	Unknown(()),
}

impl HttpVersion {
	pub fn as_str(&self) -> &'static str {
		match self {
			HttpVersion::Http10 => "HTTP/1.0",
			HttpVersion::Http11 => "HTTP/1.1",
			HttpVersion::Http20 => "HTTP/2.0",
			HttpVersion::Http30 => "HTTP/3.0",
			HttpVersion::Unknown(_) => "UNKNOWN",
		}
	}
	
	pub fn from_str(version: &str) -> Self {
		match version {
			"HTTP/1.0" => HttpVersion::Http10,
			"HTTP/1.1" => HttpVersion::Http11,
			"HTTP/2.0" | "HTTP/2" => HttpVersion::Http20,
			"HTTP/3.0" | "HTTP/3" => HttpVersion::Http30,
			_ => HttpVersion::Unknown(()),
		}
	}
	
	pub fn is_supported(&self) -> bool {
		matches!(self, HttpVersion::Http10 | HttpVersion::Http11)
	}
}