use std::{
	collections::HashMap,
	error::Error,
	net::SocketAddr,
	time::Duration
};
use tokio::{
	io::{AsyncReadExt, AsyncWriteExt, BufReader},
	time::{timeout, timeout_at, Instant},
	net::TcpStream
};
use crate::{
	core::RateLimiter,
	enums::{HttpError, HttpStatusCode},
	MAX_HEADERS_SIZE,
	protocols::{HttpV11}
};
use tokio_rustls::server::TlsStream;

pub async fn handle(stream: TlsStream<TcpStream>, addr: SocketAddr) -> Result<(), Box<dyn Error>> {
	let ip: String = addr.ip().to_string();
	RateLimiter::add(ip.clone());
	
	let mut reader: BufReader<TlsStream<TcpStream>> = BufReader::new(stream);
	if RateLimiter::is_blocked(&ip) {
		throw_error_and_shutdown(reader.get_mut(), HttpStatusCode::TooManyRequests).await;
		return Err(Box::new(HttpError::TooManyRequests));
	}
	
	let mut header_buffer: [u8; MAX_HEADERS_SIZE] = [0u8; MAX_HEADERS_SIZE];
	
	let total_deadline: Instant = Instant::now() + Duration::from_secs(12);
	let chunk_deadline = Duration::from_secs(6);
	
	let bytes_read = timeout_at(
		total_deadline,
		timeout(chunk_deadline, reader.read(&mut header_buffer))
	).await???;
	
	if bytes_read == 0 {
		return Err(Box::new(HttpError::ConnectionClosed));
	}
	
	reader.into_inner().write_all(HttpV11::from_status_code(HttpStatusCode::Ok, HashMap::new()).as_bytes()).await?;
	
	Ok(())
}

async fn throw_error_and_shutdown(stream: &mut TlsStream<TcpStream>, status_code: HttpStatusCode) {
	if let Ok(_) = stream.write_all(HttpV11::from_status_code(status_code, HashMap::new()).as_bytes()).await {
		match stream.shutdown().await {
			Ok(_) => (),
			Err(e) => {
				eprintln!("Failed to shutdown stream: {}", e.to_string());
			}
		}
	}
}