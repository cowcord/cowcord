#[derive(Clone, Debug, PartialEq)]
pub struct Token(pub String);

impl Token {
	pub fn is_valid(&self) -> bool {
		self.0.len() == 70
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
