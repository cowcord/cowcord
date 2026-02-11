use base64::Engine;
use base64::prelude::BASE64_STANDARD;

#[derive(Clone, Debug, PartialEq)]
pub struct Token(pub String);

impl Token {
	pub fn is_valid(&self) -> bool {
		let mut sections = self.0.split(".");

		// decode first from base64
		// then cast the string to a int
		// if all goes well then assume its a valid snowflake
		// todo: when UserId type is added, use a `is_valid` for that instead of just assuming any number is fine
		let section1_valid = sections
			.next()
			.and_then(|s| BASE64_STANDARD.decode(s).ok())
			.and_then(|decoded| String::from_utf8(decoded).ok())
			.and_then(|s| s.parse::<u32>().ok())
			.is_some();

		// verify theres exactly 2 more non-empty sections
		section1_valid
			&& sections.next().is_some_and(|s| !s.is_empty())
			&& sections.next().is_some_and(|s| !s.is_empty())
			&& sections.next().is_none()
	}
}

impl From<&str> for Token {
	fn from(value: &str) -> Self {
		Token(value.to_owned())
	}
}

impl From<String> for Token {
	fn from(value: String) -> Self {
		Token(value)
	}
}
