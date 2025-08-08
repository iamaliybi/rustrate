use std::collections::HashMap;
use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use crate::enums::{HttpError, HttpMethod, HttpVersion};
use crate::protocols::{HttpV10, HttpV11};
use crate::traits::HttpProtocol;
use crate::utils::helper::http_date_string;

pub struct Response {
	pub stream: TcpStream,
	pub method: HttpMethod,
	pub path: String,
	pub http_version: HttpVersion,
	pub headers: HashMap<String, String>,
}

impl Response {
	pub async fn builder(mut self: Self) -> Result<(), Box<dyn Error>> {
		if self.method.is_supported() == false {
			self.http_method_not_supported().await;
			return Ok(());
		}
		
		match self.http_version {
			HttpVersion::Http10 => {
				HttpV10::parse(self.headers.clone(), &mut self.stream).await;
			}
			
			HttpVersion::Http11 => {
				HttpV11::parse(self.headers.clone(), &mut self.stream).await;
			}
			
			_ => {
				self.http_version_not_supported().await;
				return Err(Box::new(HttpError::UnsupportedVersion));
			}
		}
		
		Ok(())
	}
	
	async fn http_version_not_supported(&mut self) {
		let reason = String::from("HTTP Version Not Supported");
		let res = format!(
			"HTTP/1.1 505 HTTP Version Not Supported\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: keep-alive\r\nDate: {}\r\nServer: RustServer/1.0.0\r\n\r\n{}",
			reason.len(),
			http_date_string(),
			reason
		);
		self.stream.write_all(res.as_bytes()).await.unwrap();
	}
	
	async fn http_method_not_supported(&mut self) {
		let reason = String::from("Method Not Allowed");
		let res = format!(
			"HTTP/1.1 405 Method Not Allowed\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: keep-alive\r\nDate: {}\r\nServer: RustServer/1.0.0\r\n\r\n{}",
			reason.len(),
			http_date_string(),
			reason
		);
		self.stream.write_all(res.as_bytes()).await.unwrap();
	}
}