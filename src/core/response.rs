use std::error::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use crate::core::HttpRequest;
use crate::enums::{HttpError, HttpVersion};
use crate::protocols::{HttpV10, HttpV11};
use crate::traits::HttpProtocol;

pub struct HttpResponse {
	pub keep_connection_alive: bool,
	pub body: Vec<u8>,
}

impl HttpResponse {
	pub async fn new(req: HttpRequest) -> Result<Self, Box<dyn Error>> {
		let version = req.version.clone();
		
		let (keep_alive, body): (bool, Vec<u8>) = match version {
			HttpVersion::Http10 => {
				let body = HttpV10::handle(req).await?;
				(false, body)
			},
			HttpVersion::Http11 => {
				let keep_alive = req.headers.get("Connection").map(|v| v.eq_ignore_ascii_case("keep-alive")).unwrap_or(false);
				let body = HttpV11::handle(req).await?;
				(keep_alive, body)
			},
			_ => return Err(Box::new(HttpError::UnsupportedVersion)),
		};
		
		Ok(
			Self {
				keep_connection_alive: keep_alive,
				body
			}
		)
	}
	
	pub async fn send(self, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
		stream.write_all(&self.body).await?;
		
		if self.keep_connection_alive == false {
			stream.shutdown().await?;
		}
		
		Ok(())
	}
}