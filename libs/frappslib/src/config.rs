use std::env;

pub fn base_url() -> String {
	env::var("FLAPPS_BASE_URL").unwrap_or("https://resco.flapps.com".to_string())
}
