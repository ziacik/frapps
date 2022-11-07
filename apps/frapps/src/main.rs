pub mod config;
pub mod credentials;

use crate::{
	config::base_url,
	credentials::{delete_credentials, retrieve_credentials, save_credentials},
};
use frappslib::{
	absences::get_absences,
	error::{generic_error, GenericError},
	login::login,
};
use regex::Regex;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), GenericError> {
	env_logger::init();

	let url = base_url().ok_or(generic_error("Please provide flapps / xperience organization url. Use a FRAPPS_URL environment variable."))?;
	let organization = parse_organization_from_url(&url)?;

	println!("Using flapps at {}.", url);

	let client = get_credentials_and_login(&url, &organization).await?;

	let absences = get_absences(&url, &client).await?;
	println!("{:#?}", absences);

	Ok(())
}

fn parse_organization_from_url(url: &str) -> Result<String, GenericError> {
	let organization_regex = Regex::new(r"^.*//(?P<org>[^.]+)").unwrap();
	let organization = organization_regex
		.captures(&url)
		.and_then(|cap| cap.name("org").map(|org| org.as_str().to_string()))
		.ok_or(GenericError("Unable to parse organization. Please provide proper flapps / xperience organization url (e.g. https://someorg.flapps.com).".to_string(), "".to_string()));

	organization
}

async fn get_credentials_and_login(url: &str, organization: &str) -> Result<Client, GenericError> {
	let credentials = retrieve_credentials(&organization)?;

	match login(url, &credentials).await {
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
