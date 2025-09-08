use serde::{Deserialize, Serialize};

use crate::api::websocket::GatewayEncoding;
use crate::utils::url::ToStringQuery;

pub fn WEBSOCKET(query: WebSocketQueryParams) -> String {
	format!("wss://gateway.discord.com{}", query.to_string_query())
}

#[derive(Serialize, Deserialize)]
pub struct WebSocketQueryParams {
	/// API Version to use
	pub v: u8,
	/// The encoding of received gateway packets
	pub encoding: GatewayEncoding,
	/// The optional transport compression of gateway packets
	#[serde(skip_serializing_if = "Option::is_none")]
	pub compress: Option<String>,
}
