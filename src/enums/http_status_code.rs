pub enum HttpStatusCode {
	Ok,
	Created,
	NoContent,
	BadRequest,
	Unauthorized,
	Forbidden,
	NotFound,
	Timeout,
	TooManyRequests,
	InternalServerError,
	NotImplemented,
	BadGateway,
	ServiceUnavailable,
	Unknown(u16),
}

impl HttpStatusCode {
	pub fn code(&self) -> u16 {
		match self {
			HttpStatusCode::Ok => 200,
			HttpStatusCode::Created => 201,
			HttpStatusCode::NoContent => 204,
			HttpStatusCode::BadRequest => 400,
			HttpStatusCode::Unauthorized => 401,
			HttpStatusCode::Forbidden => 403,
			HttpStatusCode::NotFound => 404,
			HttpStatusCode::Timeout => 408,
			HttpStatusCode::TooManyRequests => 429,
			HttpStatusCode::InternalServerError => 500,
			HttpStatusCode::NotImplemented => 501,
			HttpStatusCode::BadGateway => 502,
			HttpStatusCode::ServiceUnavailable => 503,
			HttpStatusCode::Unknown(code) => *code,
		}
	}
	
	pub fn reason(&self) -> &'static str {
		match self {
			HttpStatusCode::Ok => "OK",
			HttpStatusCode::Created => "Created",
			HttpStatusCode::NoContent => "No Content",
			HttpStatusCode::BadRequest => "Bad Request",
			HttpStatusCode::Unauthorized => "Unauthorized",
			HttpStatusCode::Forbidden => "Forbidden",
			HttpStatusCode::NotFound => "Not Found",
			HttpStatusCode::Timeout => "Request Timeout",
			HttpStatusCode::TooManyRequests => "Too Many Requests",
			HttpStatusCode::InternalServerError => "Internal Server Error",
			HttpStatusCode::NotImplemented => "Not Implemented",
			HttpStatusCode::BadGateway => "Bad Gateway",
			HttpStatusCode::ServiceUnavailable => "Service Unavailable",
			HttpStatusCode::Unknown(_) => "Unknown",
		}
	}
}