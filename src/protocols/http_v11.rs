use std::collections::HashMap;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use crate::traits::HttpProtocol;
use crate::utils::helper::http_date_string;

pub struct HttpV11 {}

impl HttpProtocol for HttpV11 {
	async fn parse(params: HashMap<String, String>, stream: &mut TcpStream) {
		let res = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 11\r\nConnection: keep-alive\r\nDate: {}\r\nServer: RustServer/1.0.0\r\n\r\nHello World", http_date_string());
		stream.write_all(res.as_bytes()).await.unwrap();
	}
}