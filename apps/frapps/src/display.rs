use tabled::{Style, Table, Tabled};

pub(crate) fn display<T: Tabled>(table: &Vec<T>) -> String {
	Table::new(table).with(Style::sharp()).to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Tabled)]
	struct Something {
		name: String,
		rating: usize,
	}

	impl Something {
		fn new(name: &str, rating: usize) -> Self {
			Self {
				name: name.to_string(),
				rating,
			}
		}
	}

	#[test]
	fn split_user_password_splits_user_password_and_returns_credentials() {
		let input = vec![Something::new("Wednesday", 1), Something::new("Enid", 9)];
		let actual = display(&input);
		let expected = "┌───────────┬────────┐\n\
							  │ name      │ rating │\n\
							  ├───────────┼────────┤\n\
							  │ Wednesday │ 1      │\n\
							  │ Enid      │ 9      │\n\
							  └───────────┴────────┘";
		assert_eq!(&actual, &expected);
	}
}
