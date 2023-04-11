use crate::{
	absences::Absence,
	error::{from_reqwest, GenericError},
};
use chrono::{format::Item, NaiveDate};
use reqwest::Client;
use scraper::{ElementRef, Html, Selector};
use tabled::Tabled;

#[derive(Debug, PartialEq, Tabled)]
pub struct Person {
	name: String,
}

#[derive(Debug, PartialEq)]
pub struct DatedAbsence {
	date: NaiveDate,
	absence: Absence,
}

#[derive(Debug, PartialEq)]
pub struct PersonCalendar {
	person: Person,
	absences: Vec<DatedAbsence>,
}

pub async fn get_calendar(
	url: &str,
	logged_in_client: &Client,
) -> Result<Vec<PersonCalendar>, GenericError> {
	let url = format!("{}/web/timesystem?__mvcevent=absenceMonthlyAxis", url);
	let response = logged_in_client
		.get(url)
		.send()
		.await
		.map_err(|e| from_reqwest("Unable to get calendar", e))?;
	let text = response
		.text()
		.await
		.map_err(|e| GenericError("Adsf".to_string(), "sfd".to_string()))?;

	let parsed = parse(&text);

	println!("{}", text);
	Result::Ok(parsed)
}

fn parse(text: &String) -> Vec<PersonCalendar> {
	let document = Html::parse_document(text);

	let person_selector = Selector::parse(".day_graph_name").unwrap();
	let person_elements: Vec<ElementRef> = document.select(&person_selector).collect();
	let personCalendars: Vec<PersonCalendar> = person_elements
		.iter()
		.map(|person_element| PersonCalendar {
			person: Person {
				name: person_element.text().next().unwrap().to_string(),
			},
			absences: retrieve_absences(person_element),
		})
		.collect();

	let day_selector = Selector::parse(".day-cell").unwrap();
	let days: Vec<_> = document.select(&day_selector).collect();
	personCalendars
}

fn retrieve_absences(person_element: &ElementRef) -> Vec<DatedAbsence> {
	let calendar_element = person_element.parent().and_then(ElementRef::wrap).unwrap();
	let selector = Selector::parse(".day-cell").unwrap();
	let daycell_elements = calendar_element.select(&selector);

	daycell_elements
		.into_iter()
		.enumerate()
		.map(|(i, daycell_element)| to_dated_absence(i, daycell_element))
		.collect()

	// println!("{:?}", daycell_elements.iter().next().unwrap().html());

	// vec![]
}

fn to_dated_absence(i: usize, daycell_element: ElementRef) -> DatedAbsence {
	DatedAbsence {
		date: NaiveDate::from_ymd(2023, 3, (i + 1).try_into().unwrap()),
		absence: Absence {
			name: "kvak".to_string(),
			absence_type: "asdf".to_string(),
			until: NaiveDate::from_ymd(2023, 3, 3),
		},
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use mockito::{mock, server_url};

	fn mock_server() -> mockito::Mock {
		mock("GET", "/web/timesystem?__mvcevent=absenceMonthlyAxis")
			.with_status(200)
			.with_header("content-type", "text/html")
			.with_body(
				r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
<html>
  <head>
    <title>XperienceHR</title>
  </head>

  <body>
    <table>
      <tr>
        <td>&nbsp;</td>
        <td>Meno</td>
        <td colspan="31" class="absenceCalendarHeader monthName">
          janu&#225;ra 2023
        </td>
      </tr>
      <tr class="absenceCalendarHeader">
        <td class="day_graph_day">1</td>
        <td class="day_graph_day curent-week">2</td>
        <td class="day_graph_day curent-week day_graph_today">3</td>
        <td class="day_graph_day curent-week">4</td>
        <td class="day_graph_day">5</td>
        <td class="day_graph_day last-day_graph_day">6</td>
      </tr>
      <tr>
        <td class="tdCalendarPhoto">
          <span class="photo photo-list"><img src="/web/timesystem?__mvcevent=content&amp;__ajax_method=getDefaultAvatar" alt="Andris Juraj" /></span>
        </td>
        <td class="day_graph_name">Andris Juraj<a name="275083" /></td>
        <td class="day-cell weekend">
          <div
            class="tooltip day_graph_item_div holiday"
            title="De&#328; vzniku Slovenskej republiky"
          ></div>
        </td>
        <td class="day-cell curent-week" />
        <td class="day-cell curent-week">
          <div
            class="tooltip day_graph_item_div half_day gradient-blue"
            title="[12.1.2023 - 12.1.2023] 0,50 dn&#237;  - otvoren&#225; (pr&#225;ca z domu)"
          >
            HO 4
          </div>
          <div
            class="tooltip day_graph_item_div half_day gradient-blue"
            title="[12.1.2023 - 12.1.2023] 0,50 dn&#237;  - otvoren&#225; (Dovolenka)"
          >
            D 4
          </div>
        </td>
        <td class="day-cell curent-week">
          <div
            class="tooltip day_graph_item_div gradient-blue"
            title="[13.1.2023 - 13.1.2023] 1,00 de&#328;  - otvoren&#225; (pr&#225;ca z domu)"
          >
            HO 8
          </div>
        </td>
        <td class="day-cell weekend" />
        <td class="day-cell last-day_graph_day" />
      </tr>
    </table>
  </body>
</html>
"#,
			)
			.create()
	}

	#[tokio::test]
	async fn should_get_calendar() {
		let _server = mock_server();
		let client = Client::new();

		let expected = vec![PersonCalendar {
			person: Person {
				name: "Andris Juraj".to_string(),
			},
			absences: vec![
				DatedAbsence {
					date: NaiveDate::from_ymd(2023, 1, 3),
					absence: Absence {
						name: String::from("Andris Juraj"),
						until: NaiveDate::from_ymd(2023, 1, 3),
						absence_type: String::from("práca z domu"),
					},
				},
				DatedAbsence {
					date: NaiveDate::from_ymd(2023, 1, 3),
					absence: Absence {
						name: String::from("Andris Juraj"),
						until: NaiveDate::from_ymd(2023, 1, 3),
						absence_type: String::from("Dovolenka"),
					},
				},
				DatedAbsence {
					date: NaiveDate::from_ymd(2023, 1, 4),
					absence: Absence {
						name: String::from("Andris Juraj"),
						until: NaiveDate::from_ymd(2023, 1, 4),
						absence_type: String::from("práca z domu"),
					},
				},
			],
		}];

		let actual = get_calendar(&server_url(), &client).await.ok().unwrap();

		assert_eq!(&actual, &expected);
	}
}
