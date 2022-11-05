use log::error;

use frappslib::{credentials::Credentials, error::GenericError};
use rpassword::prompt_password;
use rprompt::prompt_reply_stdout;

pub fn retrieve_credentials(organization: &str) -> Result<Credentials, GenericError> {
	match try_get_from_keyring(organization) {
		Some(credentials) => Ok(credentials),
		None => try_get_from_prompt(organization),
	}
}

pub fn save_credentials(credentials: &Credentials) {
	let entry = keyring::Entry::new("frapps", &credentials.0);
	let user_password = format!("{}|{}", credentials.1, credentials.2);
	let result = entry.set_password(&user_password);

	match result {
		Ok(_) => (),
		Err(e) => error!("Unable to save credentials into keyring ({}).", e),
	}
}

pub fn delete_credentials(organization: &str) {
	let entry = keyring::Entry::new("frapps", organization);
	let result = entry.delete_password();

	match result {
		Ok(_) => (),
		Err(e) => error!("Unable to save credentials into keyring ({}).", e),
	}
}

fn try_get_from_keyring(organization: &str) -> Option<Credentials> {
	let entry = keyring::Entry::new("frapps", organization);
	let maybe_user_password = entry.get_password();

	match maybe_user_password {
		Ok(user_password) => split_user_password(organization, &user_password),
		Err(_) => None,
	}
}

fn split_user_password(organization: &str, user_password: &str) -> Option<Credentials> {
	user_password
		.split_once('|')
		.and_then(|user_password_split| {
			Some(Credentials(
				organization.to_string(),
				user_password_split.0.to_string(),
				user_password_split.1.to_string(),
			))
		})
}

fn try_get_from_prompt(organization: &str) -> Result<Credentials, GenericError> {
	println!("Credentials not found in keyring, please provide user name and password.");
	let user = map_io_error(prompt_reply_stdout("User: "))?;
	let password = map_io_error(prompt_password("Password: "))?;
	Ok(Credentials(organization.to_string(), user, password))
}

fn map_io_error(some: Result<String, std::io::Error>) -> Result<String, GenericError> {
	some.map_err(|e| GenericError("Unable to retrieve credentials.".to_string(), e.to_string()))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn split_user_password_splits_user_password_and_returns_credentials() {
		let actual = split_user_password("some-org", "some-user|some-password");
		let expected = Some(Credentials(
			"some-org".to_string(),
			"some-user".to_string(),
			"some-password".to_string(),
		));
		assert_eq!(actual, expected);
	}

	#[test]
	fn split_user_password_correctly_treats_splitter_char_used_in_password() {
		let actual = split_user_password("some-org", "some-user|some|password");
		let expected = Some(Credentials(
			"some-org".to_string(),
			"some-user".to_string(),
			"some|password".to_string(),
		));
		assert_eq!(actual, expected);
	}

	#[test]
	fn split_user_password_returns_none_if_user_password_is_unsplittable() {
		let actual = split_user_password("some-org", "something-arbitrary");
		let expected = None;
		assert_eq!(actual, expected);
	}
}
