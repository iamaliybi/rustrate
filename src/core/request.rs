use std::{ collections::HashMap, error::Error };
use crate::{
	enums::{HttpError, HttpMethod, HttpVersion},
	utils::helper::sanitize_header_value,
	MAX_HEADERS_LENGTH
};

pub struct HttpRequest {
	pub path: String,
	pub method: HttpMethod,
	pub version: HttpVersion,
	pub headers: HashMap<String, String>,
	pub body: Vec<u8>,
}

impl HttpRequest {
	pub async fn new(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
		/*
		 * CR = 0x0D = /r
		 * LF = 0x0A = /n
		 * SP = 0x20
		 * Delimiter = /r/n/r/n
		 */
		
		let mut i: usize = 0;
		let length = bytes.len(); // MAX_HEADERS_SIZE
		
		// 1. Request Line
		let (method, path, version): (HttpMethod, String, HttpVersion) = {
			let mut method: Option<HttpMethod> = None;
			let mut path: Option<String> = None;
			let mut version: Option<HttpVersion> = None;
			while i < length - 1 {
				if bytes[i] == 0x0D && bytes[i + 1] == 0x0A {
					(method, path, version) = Self::parse_request_line(&bytes[..i])?;
					break;
				}
				
				i += 1;
			}
			
			// Validate method, path, and version
			if method.is_none() || path.is_none() || version.is_none() {
				return Err(Box::new(HttpError::RequestLineNotFound));
			}
			
			(method.unwrap(), path.unwrap(), version.unwrap())
		};
		
		// 4. Headers
		let mut j = i;
		let mut eoh = false; // END OF HEADERS
		let mut length_of_headers = 0;
		let mut headers: HashMap<String, String> = HashMap::new();
		while i < length - 3 {
			if bytes[i] == 0x0D && bytes[i + 1] == 0x0A {
				if let Some((key, value)) = Self::parse_header(&bytes[j..i]) {
					headers.insert(key, value);
				}
				
				if bytes[i + 2] == 0x0D && bytes[i + 3] == 0x0A {
					eoh = true;
					break;
				}
				
				j = i;
				i += 2;
				length_of_headers += 1;
			} else {
				i += 1;
			}
		}
		
		if length_of_headers > MAX_HEADERS_LENGTH || eoh == false {
			return Err(Box::new(HttpError::HeadersTooLarge));
		}
		
		// 5. Body
		i += 4; // CR LF
		let body: Vec<u8> = if method.has_body() { bytes[i..].to_vec() } else { Vec::new() };
		
		Ok(
			Self {
				path,
				method,
				version,
				headers,
				body: body.to_vec(),
			}
		)
	}
	
	pub fn set_body(&mut self, body: Vec<u8>) {
		self.body = body;
	}
	
	pub fn content_length(&self) -> usize {
		self.headers.get("Content-Length").unwrap_or(&String::from("0")).parse::<usize>().unwrap_or(0)
	}
	
	fn parse_request_line(as_bytes: &[u8]) -> Result<(Option<HttpMethod>, Option<String>, Option<HttpVersion>), Box<dyn Error>> {
		if let Ok(str) = std::str::from_utf8(as_bytes) {
			let mut parts = str.splitn(3, ' ');
			
			let method: HttpMethod;
			if let Some(v) = parts.next() {
				method = HttpMethod::from_str(v);
				if method.is_supported() == false {
					return Err(Box::new(HttpError::UnsupportedMethod));
				}
			} else {
				return Err(Box::new(HttpError::MethodNotFound));
			}
			
			let path: String;
			if let Some(v) = parts.next() {
				if let Ok(decoded_path) = urlencoding::decode(v) {
					path = decoded_path.to_string();
				} else {
					return Err(Box::new(HttpError::DecodeUrlFailed));
				}
			} else {
				return Err(Box::new(HttpError::UrlNotFound));
			}
			
			let version: HttpVersion;
			if let Some(v) = parts.next() {
				version = HttpVersion::from_str(v);
				if version.is_supported() == false {
					return Err(Box::new(HttpError::UnsupportedVersion));
				}
			} else {
				return Err(Box::new(HttpError::VersionNotFound));
			}
			
			return Ok((Some(method), Some(path), Some(version)));
		}
		
		Ok((None, None, None))
	}
	
	fn parse_header(as_bytes: &[u8]) -> Option<(String, String)> {
		if let Ok(header) = std::str::from_utf8(&as_bytes) {
			let mut parts = header.splitn(2, ':');
			
			if let Some(key) = parts.next() {
				if let Some(value) = parts.next() {
					return Some((String::from(key.trim()), String::from(sanitize_header_value(value.trim()))));
				}
			}
		}
		
		None
	}
}