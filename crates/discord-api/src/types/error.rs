use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormError {
	pub code: String,
	pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormErrorWrapper {
	#[serde(rename = "_errors")]
	pub errors: Vec<FormError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FormErrors {
	Wrapper(FormErrorWrapper),
	Map(HashMap<String, FormErrors>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
	pub code: i32,
	pub message: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub errors: Option<FormErrors>,
}
