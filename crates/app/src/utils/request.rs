use dioxus::prelude::use_navigator;
use discord_api::{ApiResponse, ApiVerion, CDN_URL, DISCORD_URL};
use reqwest::{Client, RequestBuilder, Response};
use serde::Serialize;
use serde::de::DeserializeOwned;
use tokio_tungstenite::tungstenite::Bytes;

use crate::utils::token::load_token;

pub struct RequestClient {
	client: Client,
	api_base: String,
	no_auth: bool,
}

pub enum BaseUrl {
	Discord,
	DiscordCdn,
}

impl RequestClient {
	pub fn new(
		base: BaseUrl,
		no_auth: bool,
	) -> Self {
		let api_base = match base {
			| BaseUrl::Discord => format!("{}/api/{:?}", DISCORD_URL, ApiVerion::v9),
			| BaseUrl::DiscordCdn => CDN_URL.to_string(),
		};

		RequestClient {
			client: Client::new(),
			api_base,
			no_auth,
		}
	}

	pub async fn post<T>(
		&self,
		endpoint: &str,
		body: Option<&T>,
	) -> Result<Response, Box<dyn std::error::Error>>
	where
		T: Serialize,
	{
		let url = format!("{}{}", self.api_base, endpoint);
		let mut request = self.client.post(&url).add_headers(self.no_auth)?;

		if let Some(body) = body {
			request = request.json(body);
		}

		Ok(request.send().await?)
	}

	pub async fn get<T>(
		&self,
		endpoint: &str,
		body: Option<&T>,
	) -> Result<Response, Box<dyn std::error::Error>>
	where
		T: Serialize,
	{
		let url = format!("{}{}", self.api_base, endpoint);
		let mut request = self.client.get(&url).add_headers(self.no_auth)?;

		if let Some(body) = body {
			request = request.json(body);
		}

		Ok(request.send().await?)
	}

	pub async fn get_bytes(
		&self,
		endpoint: &str,
	) -> Result<Bytes, Box<dyn std::error::Error>> {
		let url = format!("{}{}", self.api_base, endpoint);
		let request = self.client.get(&url).add_headers(self.no_auth)?;

		let response = request.send().await?;
		let status = response.status();

		if status.is_success() {
			Ok(response.bytes().await?)
		} else {
			Err(format!("Request failed with status: {}", status,).into())
		}
	}

	pub async fn delete<T>(
		&self,
		endpoint: &str,
		body: Option<&T>,
	) -> Result<Response, Box<dyn std::error::Error>>
	where
		T: Serialize,
	{
		let url = format!("{}{}", self.api_base, endpoint);
		let mut request = self.client.delete(&url).add_headers(self.no_auth)?;

		if let Some(body) = body {
			request = request.json(body);
		}

		Ok(request.send().await?)
	}

	pub async fn put<T>(
		&self,
		endpoint: &str,
		body: Option<&T>,
	) -> Result<Response, Box<dyn std::error::Error>>
	where
		T: Serialize,
	{
		let url = format!("{}{}", self.api_base, endpoint);
		let mut request = self.client.put(&url).add_headers(self.no_auth)?;

		if let Some(body) = body {
			request = request.json(body);
		}

		Ok(request.send().await?)
	}

	pub async fn patch<T>(
		&self,
		endpoint: &str,
		body: Option<&T>,
	) -> Result<Response, Box<dyn std::error::Error>>
	where
		T: Serialize,
	{
		let url = format!("{}{}", self.api_base, endpoint);
		let mut request = self.client.patch(&url).add_headers(self.no_auth)?;

		if let Some(body) = body {
			request = request.json(body);
		}

		Ok(request.send().await?)
	}
}

pub trait RequestBuilderExt {
	fn add_headers(
		self,
		no_auth: bool,
	) -> Result<RequestBuilder, Box<dyn std::error::Error>>;
}

impl RequestBuilderExt for RequestBuilder {
	fn add_headers(
		self,
		no_auth: bool,
	) -> Result<RequestBuilder, Box<dyn std::error::Error>> {
		let nav = use_navigator();
		let token = load_token()?;
		let mut builder = self.header("Origin", DISCORD_URL);

		if !no_auth {
			if let Some(token) = token
				&& token.is_valid()
			{
				builder = builder.header("Authorization", token.0);
			} else {
				nav.replace("/login");
				return Err("Authorization token is missing or is invalid".into());
			}
		}

		Ok(builder)
	}
}

pub trait AutoHandle {
	/// Simple error handling for requests giving the deserialized body on `2xx`, otherwise throwing an error
	async fn with_auto_handle<T: DeserializeOwned>(
		self
	) -> Result<ApiResponse<T>, Box<dyn std::error::Error>>;
}

impl AutoHandle for Response {
	async fn with_auto_handle<T: DeserializeOwned>(
		self
	) -> Result<ApiResponse<T>, Box<dyn std::error::Error>> {
		let status = self.status();
		let response_text = self.text().await?;

		if status.is_success() {
			let result: ApiResponse<T> = serde_json::from_str(&response_text)?;
			Ok(result)
		} else {
			Err(format!(
				"Request failed with status: {}. Response: {}",
				status, response_text
			)
			.into())
		}
	}
}
