#[derive(Debug)]
pub enum HttpMethod {
	Get,
	Post,
	Put,
	Patch,
	Delete,
	Head,
	Unknown(()),
}

impl PartialEq for HttpMethod {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(HttpMethod::Get, HttpMethod::Get) => true,
			(HttpMethod::Post, HttpMethod::Post) => true,
			(HttpMethod::Put, HttpMethod::Put) => true,
			(HttpMethod::Patch, HttpMethod::Patch) => true,
			(HttpMethod::Delete, HttpMethod::Delete) => true,
			(HttpMethod::Head, HttpMethod::Head) => true,
			_ => false,
		}
	}
}

impl HttpMethod {
	pub fn from_str(version: &str) -> Self {
		match version.trim().to_ascii_uppercase().as_str() {
			"GET" => HttpMethod::Get,
			"POST" => HttpMethod::Post,
			"PUT" => HttpMethod::Put,
			"PATCH" => HttpMethod::Patch,
			"DELETE" => HttpMethod::Delete,
			"HEAD" => HttpMethod::Head,
			_ => HttpMethod::Unknown(()),
		}
	}
	
	pub fn is_supported(&self) -> bool {
		matches!(self, HttpMethod::Get | HttpMethod::Post | HttpMethod::Put | HttpMethod::Patch | HttpMethod::Delete | HttpMethod::Head)
	}
	
	pub fn has_body(&self) -> bool {
		matches!(self, HttpMethod::Post | HttpMethod::Put | HttpMethod::Patch)
	}
}