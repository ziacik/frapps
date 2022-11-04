pub struct Credentials(pub String, pub String, pub String);

impl Credentials {
	pub fn to_form(self: &Credentials) -> [(&str, &str); 3] {
		[
			("instance", &self.0),
			("login", &self.1),
			("password", &self.2),
		]
	}
}
