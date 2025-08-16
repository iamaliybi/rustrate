use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};
use std::time::{Duration, Instant};
use tokio::{time::interval};
use crate::MAX_REQUEST_PER_MINUTE;

static RATE_LIMITER: LazyLock<RwLock<HashMap<String, (u8, Instant)>>> = LazyLock::new(|| RwLock::new(HashMap::new()));

pub struct RateLimiter;

impl RateLimiter {
	pub fn add(ip: String) {
		if let Ok(mut map) = RATE_LIMITER.write() {
			let (counter, time): (u8, Instant) = {
				if let Some((counter, last_time)) = map.get(&ip) {
					let new_counter = if RateLimiter::is_alive(*last_time) {
						counter.saturating_add(1)
					} else {
						1
					};
					
					(new_counter, Instant::now())
				}
				else {
					(1, Instant::now())
				}
			};

			map.insert(ip, (counter, time));
		}
	}
	
	pub fn is_blocked(ip: &String) -> bool {
		if let Some((count, last_time)) = RateLimiter::get(ip) {
			if RateLimiter::is_alive(last_time) && count >= MAX_REQUEST_PER_MINUTE {
				return true;
			}
		}
		
		false
	}
	
	pub fn get(ip: &String) -> Option<(u8, Instant)> {
		if let Ok(map) = RATE_LIMITER.read() {
			if let Some(val) = map.get(ip) {
				return Some(*val);
			}
		}
		
		None
	}
	
	pub async fn cleanup() {
		let mut ticker = interval(Duration::from_secs(5));
		loop {
			ticker.tick().await;
			let now = Instant::now();
			if let Ok(mut map) = RATE_LIMITER.write() {
				map.retain(|_, &mut (_, last_time)| {
					now.duration_since(last_time) < Duration::from_secs(60)
				});
			}
		}
	}
	
	fn is_alive(time: Instant) -> bool {
		time.elapsed() < Duration::from_secs(60)
	}
}