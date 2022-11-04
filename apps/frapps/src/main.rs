use frappslib::{
	absences::get_absences, config::base_url, credentials::Credentials, error::GenericError,
	login::login,
};
use regex::Regex;
use rpassword::prompt_password;
use rprompt::prompt_reply_stdout;

#[tokio::main]
async fn main() -> Result<(), GenericError> {
	let url = base_url();

	let organization_regex = Regex::new(r"^.*//(?P<org>[^.]+)").unwrap();
	let organization = organization_regex
		.captures(&url)
		.and_then(|cap| cap.name("org").map(|org| org.as_str()))
		.unwrap_or("resco");

	println!(
		"Using flapps at {}. To change, set FLAPPS_BASE_URL. Organization: {}.",
		url, organization
	);

	let user = prompt_reply_stdout("User:").unwrap();
	let password = prompt_password("Password:").unwrap();

	let client = login(Credentials(
		organization.to_string(),
		user.to_string(),
		password,
	))
	.await?;

	let absences = get_absences(&client).await.unwrap();
	println!("{:#?}", absences);

	Ok(())
}
