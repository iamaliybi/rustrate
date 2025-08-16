use std::error::Error;
use crate::core::{HttpRequest};

pub trait HttpProtocol {
	async fn handle(request: HttpRequest) -> Result<Vec<u8>, Box<dyn Error>>;
}