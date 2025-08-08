use std::collections::HashMap;
use tokio::net::TcpStream;

pub trait HttpProtocol {
	async fn parse(params: HashMap<String, String>, stream: &mut TcpStream);
}