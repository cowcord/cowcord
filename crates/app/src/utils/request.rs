use dioxus::prelude::use_navigator;
use discord_api_types::{ApiVerion, DISCORD_URL};
use reqwest::{Client, RequestBuilder, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;
use wasm_bindgen::JsError;

use crate::utils::local_storage::LocalStorage;

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

	async fn handle_response<T>(response: Response) -> Result<T, JsError>
	where
		T: DeserializeOwned,
	{
		let status = response.status();
		let response_text = response.text().await?;

		if status.is_success() {
			let result: T = serde_json::from_str(&response_text)?;
			Ok(result)
		} else {
			Err(JsError::new(&format!(
				"Request failed with status: {}. Response: {}",
				status, response_text
			)))
		}
	}

	pub async fn post<T, R>(
		&self,
		endpoint: &str,
		body: Option<&T>,
	) -> Result<R, JsError>
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
	) -> Result<R, JsError>
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
	) -> Result<R, JsError>
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
	) -> Result<R, JsError>
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
	) -> Result<R, JsError>
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
	fn add_headers(self) -> Result<RequestBuilder, JsError>;
}

impl RequestBuilderExt for RequestBuilder {
	fn add_headers(self) -> Result<RequestBuilder, JsError> {
		if let Some(token) = LocalStorage::new()
			.map_err(|e| JsError::from(e))?
			.get_value("token")
		{
			Ok(self
				.header("Authorization", token)
				.header("Origin", DISCORD_URL))
		} else {
			use_navigator().replace("/login");
			Err(JsError::new("Authorization token is missing"))
		}
	}
}
