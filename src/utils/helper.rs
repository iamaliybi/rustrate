use std::collections::HashMap;
use chrono::Utc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::TcpStream;

pub fn http_date_string() -> String {
	let now = Utc::now();
	now.format("%a, %d %b %Y %H:%M:%S GMT").to_string()
}