use std::env;

pub fn base_url() -> Option<String> {
	env::var("FRAPPS_URL").map_or(None, |url| Some(url))
}
