use std::{
	error::Error,
	net::SocketAddr,
	time::{Duration}
};
use tokio::{
	io::{AsyncReadExt, AsyncWriteExt, BufReader},
	net::TcpStream,
	time::{Instant, timeout, timeout_at}
};
use crate::{
	core::{HttpRequest, HttpResponse},
	enums::{HttpError, HttpStatusCode},
	protocols::HttpV10,
	MAX_HEADERS_SIZE,
	MAX_BODY_SIZE
};
use crate::core::RateLimiter;

pub async fn handle(stream: TcpStream, addr: SocketAddr) -> Result<(), Box<dyn Error>> {
	let ip: String = addr.ip().to_string();
	RateLimiter::add(ip.clone());
	
	let mut reader: BufReader<TcpStream> = BufReader::new(stream);
	if RateLimiter::is_blocked(&ip) {
		throw_error_and_shutdown(reader.get_mut(), HttpStatusCode::TooManyRequests).await;
		return Err(Box::new(HttpError::TooManyRequests));
	}
	
	let mut header_buffer: [u8; MAX_HEADERS_SIZE] = [0u8; MAX_HEADERS_SIZE];
	
	let total_deadline: Instant = Instant::now() + Duration::from_secs(12);
	let chunk_deadline = Duration::from_secs(6);
	
	let read_result = timeout_at(
		total_deadline,
		timeout(chunk_deadline, reader.read(&mut header_buffer))
	).await;
	let bytes_read: usize = match read_result {
		Ok(Ok(Ok(n))) => n,
		Ok(Ok(Err(e))) => {
			throw_error_and_shutdown(reader.get_mut(), HttpStatusCode::BadRequest).await;
			return Err(Box::new(e));
		},
		_ => {
			throw_error_and_shutdown(reader.get_mut(), HttpStatusCode::Timeout).await;
			return Err(Box::new(HttpError::RequestTimeout));
		}
	};
	
	if bytes_read == 0 {
		return Err(Box::new(HttpError::ConnectionClosed));
	}
	
	let header_buffer: &[u8] = &header_buffer[..bytes_read];
	let mut req: HttpRequest = HttpRequest::new(header_buffer).await?;
	read_body(&mut reader, &mut req, total_deadline, chunk_deadline).await?;
	
	let res: HttpResponse = HttpResponse::new(req).await?;
	res.send(reader.into_inner()).await?;
	
	Ok(())
}

async fn read_body(reader: &mut BufReader<TcpStream>, req: &mut HttpRequest, total_deadline: Instant, chunk_deadline: Duration) -> Result<(), Box<dyn Error>> {
	if req.method.has_body() {
		if req.headers.contains_key("Transfer-Encoding") {
			throw_error_and_shutdown(reader.get_mut(), HttpStatusCode::NotImplemented).await;
			return Err(Box::new(HttpError::NotImplemented));
		}
		
		let content_length = req.content_length();
		if content_length == 0 {
			throw_error_and_shutdown(reader.get_mut(), HttpStatusCode::BadRequest).await;
			return Err(Box::new(HttpError::BodyNotFound));
		}
		
		if content_length > MAX_BODY_SIZE {
			throw_error_and_shutdown(reader.get_mut(), HttpStatusCode::BadRequest).await;
			return Err(Box::new(HttpError::BodyTooLarge));
		}
		
		let remain_buffer_length = content_length.saturating_sub(req.body.len());
		if remain_buffer_length > 0 {
			let mut body_buffer: Vec<u8> = Vec::with_capacity(content_length);
			body_buffer.extend_from_slice(&req.body);
			
			{
				let mut remaining_chunk: Vec<u8> = vec![0u8; remain_buffer_length];
				let read_result = timeout_at(
					total_deadline,
					timeout(chunk_deadline, reader.read_exact(&mut remaining_chunk))
				).await;
				
				match read_result {
					Ok(Ok(Ok(_))) => (),
					Ok(Ok(Err(e))) => {
						throw_error_and_shutdown(reader.get_mut(), HttpStatusCode::BadRequest).await;
						return Err(Box::new(e));
					},
					_ => {
						throw_error_and_shutdown(reader.get_mut(), HttpStatusCode::Timeout).await;
						return Err(Box::new(HttpError::RequestTimeout));
					}
				}
				
				body_buffer.extend_from_slice(&remaining_chunk);
			}
			
			req.set_body(body_buffer);
		}
	}
	
	Ok(())
}

async fn throw_error_and_shutdown(stream: &mut TcpStream, status_code: HttpStatusCode) {
	if let Ok(_) = stream.write_all(HttpV10::from_status_code(status_code).as_bytes()).await {
		match stream.shutdown().await {
			Ok(_) => (),
			Err(e) => {
				eprintln!("Failed to shutdown stream: {}", e.to_string());
			}
		}
	}
}