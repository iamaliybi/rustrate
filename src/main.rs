mod core;
mod protocols;
mod traits;
mod utils;
mod enums;

use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::net::TcpListener;
use core::Request;

const TIME_TO_LIVE: u32 = 128;
const IP_V4_PORT: u16 = 8000;

#[tokio::main]
async fn main() {
	let ipv4 = Ipv4Addr::new(127, 0, 0, 1);
	let socket_v4 = SocketAddrV4::new(ipv4, IP_V4_PORT);
	
	match TcpListener::bind(socket_v4).await {
		Ok(listener) => {
			if let Err(e) = listener.set_ttl(TIME_TO_LIVE) {
				eprintln!("Failed to set ttl: {e}");
			}
			
			loop {
				match listener.accept().await {
					Ok((stream, addr)) => {
						tokio::task::spawn(async move {
							if let Err(err) = Request::builder(stream, addr).await {
								eprintln!("Err: {}", err.to_string());
							}
						});
					}
					
					Err(e) => {
						eprintln!("{}", e.to_string());
					}
				}
			}
		}
		
		Err(e) => {
			panic!("{}", e.to_string());
		}
	}
}