use crate::{
	credentials::Credentials,
	error::{from_reqwest, GenericError},
};

use reqwest::{header::LOCATION, redirect::Policy, Client};

pub async fn login(url: &str, credentials: &Credentials) -> Result<Client, GenericError> {
	let url = format!("{}/login_check", url);
	let client = Client::builder()
		.cookie_store(true)
		.redirect(Policy::none())
		.build()
		.unwrap();

	let response = client
		.post(url)
		.form(&credentials.to_form())
		.send()
		.await
		.map_err(|e| from_reqwest("Unable to login", e))?;

	let location = response.headers().get(LOCATION);

	let location = match location {
		Some(location) => Ok(location),
		None => Err(GenericError(
			"Login error".to_string(),
			"location header missing".to_string(),
		)),
	}?;

	let location = location.to_str().map_err(|e| {
		GenericError(
			"Login error: invalid location header".to_string(),
			e.to_string(),
		)
	})?;

	let result: Result<_, _> = if location.ends_with("/") {
		Ok(client)
	} else {
		Err(GenericError("Bad login".to_string(), "".to_string()))
	};

	result
}

#[cfg(test)]
mod tests {
	use super::*;
	use mockito::{mock, server_url};
	use tokio_test::{assert_err, assert_ok};

	fn mock_login() -> [mockito::Mock; 2] {
		[
			mock("POST", "/login_check")
				.match_body("instance=company&login=anicka.krkvava&password=krkvany.zuzol")
				.with_status(201)
				.with_header("location", "https://some.base/")
				.create(),
			mock("POST", "/login_check")
				.with_status(201)
				.with_header("location", "https://some.base/login.jsp")
				.create(),
		]
	}

	#[tokio::test]
	async fn can_login() {
		let _server = mock_login();

		let actual = login(
			&server_url(),
			&Credentials(
				"company".to_string(),
				"anicka.krkvava".to_string(),
				"krkvany.zuzol".to_string(),
			),
		)
		.await;

		assert_ok!(actual);
	}

	#[tokio::test]
	async fn will_error_if_bad_login() {
		let _server = mock_login();

		let actual = login(
			&server_url(),
			&Credentials(
				"company".to_string(),
				"anicka.krkvava".to_string(),
				"bad.password".to_string(),
			),
		)
		.await;

		assert_err!(&actual);
		assert_eq!(actual.unwrap_err().0, "Bad login")
	}
}
