use std::sync::OnceLock;

use discord_api::DISCORD_URL;
use discord_api::endpoints::experiments::{
	GET_EXPERIMENT_ASSIGNMENTS,
	GetExperimentAssignmentsQuery,
	GetExperimentAssignmentsResponse,
};

use crate::utils::fingerprint;

pub static FINGERPRINT: OnceLock<String> = OnceLock::new();

pub async fn get_fingerprint() -> Result<String, Box<dyn std::error::Error>> {
	let query = GetExperimentAssignmentsQuery {
		with_guild_experiments: Some(true),
		platform: None,
	};

	let resp = reqwest::Client::new()
		.get(format!(
			"{DISCORD_URL}/api/v9{}",
			&GET_EXPERIMENT_ASSIGNMENTS(&query)
		))
		.header("Origin", format!("{DISCORD_URL}/login"))
		.send()
		.await?;

	println!("{resp:?}");

	let body: GetExperimentAssignmentsResponse = resp.json().await?;

	Ok(body.fingerprint.ok_or("fingerprint was not provided")?)
}
