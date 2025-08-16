use std::collections::HashMap;
use std::error::Error;
use crate::core::{HttpRequest};
use crate::enums::HttpStatusCode;
use crate::traits::HttpProtocol;
use crate::utils::helper::http_date_string;

pub struct HttpV11;

impl HttpProtocol for HttpV11 {
	async fn handle(req: HttpRequest) -> Result<Vec<u8>, Box<dyn Error>> {
		let res = Self::from_status_code(HttpStatusCode::NotFound, req.headers).into_bytes();
		Ok(res)
	}
}

impl HttpV11 {
	pub fn from_status_code(status: HttpStatusCode, headers: HashMap<String, String>) -> String {
		let keep_alive: bool = headers.get("Connection").map(|v| v.eq_ignore_ascii_case("keep-alive")).unwrap_or(false);
		let connection_header = if keep_alive == true { "keep-alive" } else { "close" };
		let body = status.reason();
		
		format!(
			"HTTP/1.1 {} {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: {}\r\nDate: {}\r\nServer: RustRate/1.0.0\r\n\r\n{}",
			status.code(),
			body,
			body.len(),
			connection_header,
			http_date_string(),
			body
		)
	}
}