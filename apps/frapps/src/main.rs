pub mod credentials;

use crate::credentials::{delete_credentials, retrieve_credentials, save_credentials};
use frappslib::{absences::get_absences, config::base_url, error::GenericError, login::login};
use regex::Regex;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), GenericError> {
	env_logger::init();

	let url = base_url();
	let organization = parse_organization_from_url(&url);

	println!(
		"Using flapps at {}. To change, set FLAPPS_BASE_URL. Organization: {}.",
		url, organization
	);

	let client = get_credentials_and_login(&organization).await?;

	let absences = get_absences(&client).await?;
	println!("{:#?}", absences);

	Ok(())
}

fn parse_organization_from_url(url: &str) -> String {
	let organization_regex = Regex::new(r"^.*//(?P<org>[^.]+)").unwrap();
	let organization = organization_regex
		.captures(&url)
		.and_then(|cap| cap.name("org").map(|org| org.as_str()))
		.unwrap_or("resco");
	organization.to_string()
}

async fn get_credentials_and_login(organization: &str) -> Result<Client, GenericError> {
	let credentials = retrieve_credentials(&organization)?;

	match login(&credentials).await {
		Ok(client) => {
			save_credentials(&credentials);
			Ok(client)
		}
		Err(e) => {
			delete_credentials(&organization);
			Err(e)
		}
	}
}
