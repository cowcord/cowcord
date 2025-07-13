use std::error::Error;

use dioxus::prelude::use_navigator;
use reqwest::{Client, RequestBuilder, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;

use super::local_storage::get_value;

pub struct RequestClient {
	client:   Client,
	api_base: String,
}

pub const DISCORD: &str = "https://discord.com/";
pub const API_VERSION: &str = "9";

impl RequestClient {
	pub fn new() -> Self {
		RequestClient {
			client:   Client::new(),
			api_base: format!("/api/v{}", API_VERSION),
		}
	}

	async fn handle_response<T>(response: Response) -> Result<T, Box<dyn Error>>
	where
		T: DeserializeOwned,
	{
		let status = response.status();
		let response_text = response.text().await?;

		if status.is_success() {
			let result: T = serde_json::from_str(&response_text)?;
			Ok(result)
		} else {
			Err(format!(
				"Request failed with status: {}. Response: {}",
				status, response_text
			)
			.into())
		}
	}

	pub async fn post<T, R>(
		&self,
		endpoint: &str,
		body: Option<&T>,
	) -> Result<R, Box<dyn Error>>
	where
		T: Serialize,
		R: DeserializeOwned,
	{
		let url = format!("{}{}{}", DISCORD, self.api_base, endpoint);
		let mut request = self.client.post(&url).add_headers()?;

		if let Some(body) = body {
			request = request.json(body);
		}

		let response = request.send().await?;

		Self::handle_response(response).await
	}

	pub async fn get<T, R>(
		&self,
		endpoint: &str,
		body: Option<&T>,
	) -> Result<R, Box<dyn Error>>
	where
		T: Serialize,
		R: DeserializeOwned,
	{
		let url = format!("{}{}{}", DISCORD, self.api_base, endpoint);
		let mut request = self.client.get(&url).add_headers()?;

		if let Some(body) = body {
			request = request.json(body);
		}

		let response = request.send().await?;

		Self::handle_response(response).await
	}

	pub async fn delete<T, R>(
		&self,
		endpoint: &str,
		body: Option<&T>,
	) -> Result<R, Box<dyn Error>>
	where
		T: Serialize,
		R: DeserializeOwned,
	{
		let url = format!("{}{}{}", DISCORD, self.api_base, endpoint);
		let mut request = self.client.delete(&url).add_headers()?;

		if let Some(body) = body {
			request = request.json(body);
		}

		let response = request.send().await?;

		Self::handle_response(response).await
	}

	pub async fn put<T, R>(
		&self,
		endpoint: &str,
		body: Option<&T>,
	) -> Result<R, Box<dyn Error>>
	where
		T: Serialize,
		R: DeserializeOwned,
	{
		let url = format!("{}{}{}", DISCORD, self.api_base, endpoint);
		let mut request = self.client.put(&url).add_headers()?;

		if let Some(body) = body {
			request = request.json(body);
		}

		let response = request.send().await?;

		Self::handle_response(response).await
	}

	pub async fn patch<T, R>(
		&self,
		endpoint: &str,
		body: Option<&T>,
	) -> Result<R, Box<dyn Error>>
	where
		T: Serialize,
		R: DeserializeOwned,
	{
		let url = format!("{}{}{}", DISCORD, self.api_base, endpoint);
		let mut request = self.client.patch(&url).add_headers()?;

		if let Some(body) = body {
			request = request.json(body);
		}

		let response = request.send().await?;

		Self::handle_response(response).await
	}
}

pub trait RequestBuilderExt {
	fn add_headers(self) -> Result<RequestBuilder, Box<dyn Error>>;
}

impl RequestBuilderExt for RequestBuilder {
	fn add_headers(self) -> Result<RequestBuilder, Box<dyn Error>> {
		if let Some(token) = get_value("token") {
			Ok(self
				.header("Authorization", token)
				.header("Origin", DISCORD))
		} else {
			use_navigator().replace("/login");
			Err("Authorization token is missing".into())
		}
	}
}

/// Convert struct to url string query params
///
/// eg.
///
/// with field(s)
/// ```rust
/// let query = MyStruct {
///     value: "hi",
///     value_two: None,
///     value_three: 25
/// }
///
/// assert_eq!(to_string_query(&query), "?value=hi&value_three=25");
/// ```
/// without fields
/// ```rust
/// let query = MyStruct {}
///
/// assert_eq!(to_string_query(&query), "");
/// ```
pub fn to_string_query<T: serde::ser::Serialize>(query: &T) -> String {
	serde_urlencoded::to_string(&query)
		.map(|q| format!("?{}", q))
		.unwrap_or_default()
}
