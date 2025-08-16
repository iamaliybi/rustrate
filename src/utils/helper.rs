use std::error::Error;
use chrono::Utc;
use rustls_pki_types::{CertificateDer, PrivateKeyDer};
use rustls_pki_types::pem::PemObject;
use tokio_rustls::rustls::ServerConfig;

pub fn http_date_string() -> String {
	let now = Utc::now();
	now.format("%a, %d %b %Y %H:%M:%S GMT").to_string()
}

pub fn sanitize_header_value(value: &str) -> String {
	value.replace('\r', "").replace('\n', "")
}

pub fn load_tls_config() -> Result<ServerConfig, Box<dyn Error>> {
	let cert = CertificateDer::pem_file_iter("certs/localhost.pem")
		.unwrap()
		.map(|item| {
			item.unwrap()
		})
		.collect();

	let key = PrivateKeyDer::from_pem_file("certs/localhost-key.pem").unwrap();
	
	let mut config = ServerConfig::builder()
		.with_no_client_auth()
		.with_single_cert(cert, key)?;
	
	// config.alpn_protocols.push(b"h2".to_vec());
	config.alpn_protocols.push(b"http/1.1".to_vec());
	config.alpn_protocols.push(b"http/1.0".to_vec());
	
	Ok(config)
}