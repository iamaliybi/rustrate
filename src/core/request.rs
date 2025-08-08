use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;
use crate::core::Response;
use crate::enums::{HttpError, HttpMethod, HttpVersion};

pub struct Request {}

impl Request {
	pub async fn builder(stream: TcpStream, _addr: SocketAddr) -> Result<(), Box<dyn Error>> {
		let mut buf_stream: BufReader<TcpStream> = BufReader::new(stream);
		
		let mut params = Self::parse(&mut buf_stream).await?;
		
		let (method, path, http_version): (HttpMethod, String, HttpVersion) = {
			let method = HttpMethod::from_str(params.get(":method").unwrap());
			let path = params.get(":path").unwrap().clone();
			let http_version = HttpVersion::from_str(params.get(":version").unwrap());
			
			params.remove(":method");
			params.remove(":path");
			params.remove(":version");
			
			(method, path, http_version)
		};
		
		Response {
			stream: buf_stream.into_inner(),
			method,
			path,
			http_version,
			headers: params,
		}.builder().await?;
		
		Ok(())
	}
	
	pub async fn parse(buf_stream: &mut BufReader<TcpStream>) -> Result<HashMap<String, String>, Box<dyn Error>> {
		let mut result: HashMap<String, String> = HashMap::new();
		
		let mut request_line: String = String::new();
		let request_line_size: usize = buf_stream.read_line(&mut request_line).await?;
		
		if request_line_size == 0 {
			return Err(Box::new(HttpError::ConnectionClosed));
		}
		
		{
			request_line = String::from(request_line.trim());
			let mut parts = request_line.splitn(3, ' ');
			
			if let Some(method) = parts.next() {
				result.insert(String::from(":method"), String::from(method.to_ascii_uppercase()));
			} else {
				return Err(Box::new(HttpError::MethodNotFound));
			}
			
			if let Some(path) = parts.next() {
				if let Ok(decoded_path) = urlencoding::decode(path) {
					result.insert(String::from(":path"), String::from(decoded_path));
				} else {
					return Err(Box::new(HttpError::DecodeUrlFailed));
				}
			} else {
				return Err(Box::new(HttpError::UrlNotFound));
			}
			
			if let Some(http_version) = parts.next() {
				result.insert(String::from(":version"), String::from(http_version.to_ascii_uppercase()));
			} else {
				return Err(Box::new(HttpError::VersionNotFound));
			}
		}
		
		loop {
			let mut header: String = String::new();
			
			if let Err(_) = buf_stream.read_line(&mut header).await {
				break;
			}
			
			header = String::from(header.trim());
			if header.is_empty() {
				break;
			}
			
			let mut parts = header.splitn(2, ':');
			if let Some(key) = parts.next() {
				if let Some(value) = parts.next() {
					result.insert(String::from(key), String::from(value));
				}
			}
		}
		
		Ok(result)
	}
}