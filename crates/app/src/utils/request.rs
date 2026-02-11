use dioxus::prelude::use_navigator;
use discord_api::{ApiVerion, DISCORD_URL};
use reqwest::{Client, RequestBuilder, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::utils::token::load_token;

pub struct RequestClient {
	client: Client,
	api_base: String,
}

impl RequestClient {
	pub fn new() -> Self {
		RequestClient {
			client: Client::new(),
			api_base: format!("/api/{:?}", ApiVerion::v10),
		}
	}

	async fn handle_response<T>(response: Response) -> Result<T, Box<dyn std::error::Error>>
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
	) -> Result<R, Box<dyn std::error::Error>>
	where
		T: Serialize,
		R: DeserializeOwned,
	{
		let url = format!("{}{}{}", DISCORD_URL, self.api_base, endpoint);
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
	) -> Result<R, Box<dyn std::error::Error>>
	where
		T: Serialize,
		R: DeserializeOwned,
	{
		let url = format!("{}{}{}", DISCORD_URL, self.api_base, endpoint);
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
	) -> Result<R, Box<dyn std::error::Error>>
	where
		T: Serialize,
		R: DeserializeOwned,
	{
		let url = format!("{}{}{}", DISCORD_URL, self.api_base, endpoint);
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
	) -> Result<R, Box<dyn std::error::Error>>
	where
		T: Serialize,
		R: DeserializeOwned,
	{
		let url = format!("{}{}{}", DISCORD_URL, self.api_base, endpoint);
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
	) -> Result<R, Box<dyn std::error::Error>>
	where
		T: Serialize,
		R: DeserializeOwned,
	{
		let url = format!("{}{}{}", DISCORD_URL, self.api_base, endpoint);
		let mut request = self.client.patch(&url).add_headers()?;

		if let Some(body) = body {
			request = request.json(body);
		}

		let response = request.send().await?;

		Self::handle_response(response).await
	}
}

pub trait RequestBuilderExt {
	fn add_headers(self) -> Result<RequestBuilder, Box<dyn std::error::Error>>;
}

impl RequestBuilderExt for RequestBuilder {
	fn add_headers(self) -> Result<RequestBuilder, Box<dyn std::error::Error>> {
		let nav = use_navigator();
		let token = load_token()?;

		if let Some(token) = token
			&& token.is_valid()
		{
			Ok(self
				.header("Authorization", token.0)
				.header("Origin", DISCORD_URL))
		} else {
			nav.replace("/login");
			Err("Authorization token is missing or is invalid".into())
		}
	}
}
