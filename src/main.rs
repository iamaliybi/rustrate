use std::error::Error;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use crate::core::RateLimiter;
use crate::listener::{handle_http_connection, handle_tls_connection};
use crate::utils::helper::load_tls_config;

mod core;
mod protocols;
mod traits;
mod utils;
mod enums;
mod listener;

pub const MAX_HEADERS_SIZE: usize = 2048;
pub const MAX_HEADERS_LENGTH: usize = 25;
pub const MAX_BODY_SIZE: usize = 2_097_152; // 2MB
pub const MAX_REQUEST_PER_MINUTE: u8 = 100;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let (http_listener, tls_listener) = create_listeners().await?;
	let tls_config = load_tls_config()?;
	let acceptor = TlsAcceptor::from(Arc::new(tls_config));
	let acceptor = acceptor.clone();
	
	create_rate_limiter_cleaner();
	
	loop {
		tokio::select! {
			Ok((stream, addr)) = http_listener.accept() => {
				tokio::spawn(async move {
					if let Err(err) = handle_http_connection(stream, addr).await {
						eprintln!("{}", err.to_string());
					}
				});
			}
			
			Ok((stream, addr)) = tls_listener.accept() => {
				match acceptor.accept(stream).await {
					Ok(tls_stream) => {
						tokio::spawn(async move {
							if let Err(err) = handle_tls_connection(tls_stream, addr).await {
								eprintln!("{}", err.to_string());
							}
						});
					}
					
					Err(err) => {
						eprintln!("{}", err.to_string());
					}
				}
			}
		}
	}
}

async fn create_listeners() -> Result<(TcpListener, TcpListener), Box<dyn Error>> {
	let ipv4 = Ipv4Addr::new(127, 0, 0, 1);
	
	let http_listener = TcpListener::bind(SocketAddrV4::new(ipv4, 80)).await?;
	let tls_listener = TcpListener::bind(SocketAddrV4::new(ipv4, 443)).await?;
	
	Ok((http_listener, tls_listener))
}

fn create_rate_limiter_cleaner() {
	tokio::spawn(RateLimiter::cleanup());
}