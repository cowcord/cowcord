use std::time::Duration;

use discord_api::types::ws::remote_auth::{
	RemoteAuthGatewayClientOpCode,
	RemoteAuthGatewayServerOpCode,
};
use discord_api::{DISCORD_URL, REMOTE_AUTH_WS_URL};
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_tungstenite::tungstenite::{Error as WsError, Message};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

const TIMEOUT: Duration = Duration::from_millis(500);

pub struct RemoteAuthWsClient(WebSocketStream<MaybeTlsStream<TcpStream>>);

impl RemoteAuthWsClient {
	pub async fn connect() -> Result<Self, Box<dyn std::error::Error>> {
		let mut request = REMOTE_AUTH_WS_URL.into_client_request()?;
		request.headers_mut().insert("Origin", DISCORD_URL.parse()?);

		let (stream, _) = connect_async(request).await?;

		Ok(Self(stream))
	}

	pub async fn recv_json(
		&mut self
	) -> Result<Option<RemoteAuthGatewayServerOpCode>, Box<dyn std::error::Error>> {
		let message = match self.0.next().await {
			| Some(Ok(msg)) => msg,
			| Some(Err(e)) => return Err(e.into()),
			| None => return Ok(None),
		};

		let value = match message {
			| Message::Text(payload) => serde_json::from_str(&payload)?,
			| Message::Close(frame) => {
				return Err(format!("Remote auth gateway closed. Frame: {:?}", frame).into());
			},
			| _ => return Ok(None),
		};

		Ok(Some(value))
	}

	pub async fn send_json(
		&mut self,
		value: &impl serde::Serialize,
	) -> Result<(), Box<dyn std::error::Error>> {
		let message = serde_json::to_string(value).map(|s| Message::Text(s.into()))?;
		self.0.send(message).await?;
		Ok(())
	}

	pub async fn next(&mut self) -> Option<Result<Message, WsError>> {
		self.0.next().await
	}

	pub async fn close(
		&mut self,
		msg: Option<CloseFrame>,
	) -> Result<(), Box<dyn std::error::Error>> {
		self.0.close(msg).await?;
		Ok(())
	}

	pub async fn send_heartbeat(&mut self) -> Result<(), Box<dyn std::error::Error>> {
		self.send_json(&RemoteAuthGatewayClientOpCode::Heartbeat)
			.await
	}
}
