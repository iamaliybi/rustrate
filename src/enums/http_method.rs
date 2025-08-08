pub enum HttpMethod {
	Get,
	Post,
	Put,
	Patch,
	Delete,
	Head,
	Unknown(()),
}

impl HttpMethod {
	pub fn as_str(&self) -> &'static str {
		match self {
			HttpMethod::Get => "GET",
			HttpMethod::Post => "POST",
			HttpMethod::Put => "PUT",
			HttpMethod::Patch => "PATCH",
			HttpMethod::Delete => "DELETE",
			HttpMethod::Head => "HEAD",
			HttpMethod::Unknown(_) => "UNKNOWN",
		}
	}
	
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
		matches!(self, HttpMethod::Get)
	}
}