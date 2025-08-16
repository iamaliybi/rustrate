use std::error::Error;
use crate::core::{HttpRequest};
use crate::enums::HttpStatusCode;
use crate::traits::HttpProtocol;

pub struct HttpV10;

impl HttpProtocol for HttpV10 {
	async fn handle(_: HttpRequest) -> Result<Vec<u8>, Box<dyn Error>> {
		let res = Self::from_status_code(HttpStatusCode::NotFound).into_bytes();
		Ok(res)
	}
}

impl HttpV10 {
	pub fn from_status_code(status: HttpStatusCode) -> String {
		let body = status.reason();
		format!(
			"HTTP/1.0 {} {}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\nServer: RustRate/1.0.0\r\n\r\n{}",
			status.code(),
			body,
			body.len(),
			body
		)
	}
}