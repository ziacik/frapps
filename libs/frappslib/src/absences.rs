use crate::error::{from_parser, from_reqwest, GenericError};
use chrono::{DateTime, NaiveDate, ParseError};
use futures::Future;
use reqwest::{Client, Response};
use serde::Deserialize;
use tabled::Tabled;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct AbsenceDto {
	fullName: String,
	r#type: String,
	until: String,
}

// FIXME we should not have to derive Tabled here which is a display related stuff
#[derive(Debug, PartialEq, Tabled)]
pub struct Absence {
	name: String,
	absence_type: String,
	until: NaiveDate,
}

pub async fn get_absences(
	url: &str,
	logged_in_client: &Client,
) -> Result<Vec<Absence>, GenericError> {
	let url = format!("{}/rest/dashboard/absences", url);
	let response = logged_in_client
		.get(url)
		.send()
		.await
		.map_err(|e| from_reqwest("Unable to get absences", e))?;

	let dtos = parse_response(response)
		.await
		.map_err(|e| from_reqwest("Unable to parse absences", e))?;

	dtos.into_iter()
		.map(to_absence)
		.collect::<Result<Vec<_>, ParseError>>()
		.map_err(|e| from_parser("One or more of the absences have a parse error", e))
}

fn parse_response(
	response: Response,
) -> impl Future<Output = Result<Vec<AbsenceDto>, reqwest::Error>> {
	response.json::<Vec<AbsenceDto>>()
}

fn to_absence(dto: AbsenceDto) -> Result<Absence, ParseError> {
	let until_time = DateTime::parse_from_rfc3339(&dto.until);
	let until = until_time.map(|time| time.date_naive())?;

	Ok(Absence {
		name: dto.fullName,
		absence_type: dto.r#type,
		until,
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	use mockito::{mock, server_url};

	fn mock_server() -> mockito::Mock {
		mock("GET", "/rest/dashboard/absences")
			.with_status(200)
			.with_header("content-type", "application/json")
			.with_body(
				r#"
				[
					{
						"fullName": "Ján Mrkva",
						"type": "Dovolenka",
						"until": "2022-11-01T00:00:00.000Z",
						"photoImgSrc": "..."
					},
					{
						"fullName": "Filoména Krkvavá",
						"type": "Práca z domu",
						"until": "2022-12-05T00:00:00.000Z",
						"photoImgSrc": "..."
					}
				]"#,
			)
			.create()
	}

	#[tokio::test]
	async fn should_get_absences() {
		let _server = mock_server();
		let client = Client::new();

		let expected = vec![
			Absence {
				name: String::from("Ján Mrkva"),
				absence_type: String::from("Dovolenka"),
				until: NaiveDate::from_ymd(2022, 11, 1),
			},
			Absence {
				name: String::from("Filoména Krkvavá"),
				absence_type: String::from("Práca z domu"),
				until: NaiveDate::from_ymd(2022, 12, 5),
			},
		];

		let actual = get_absences(&server_url(), &client).await.unwrap();

		assert_eq!(&actual, &expected);
	}
}
