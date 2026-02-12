use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum RemoteAuthGatewayClientOpCode {
	/// Start a new remote auth session
	Init {
		/// The base64-encoded SPKI of the client's 2048-bit RSA-OAEP public key
		encoded_public_key: String,
	},
	/// Maintain an active WebSocket connection
	Heartbeat,
	///	Submit a cryprographic proof of the handshake
	NonceProof {
		/// The base64URL-encoded decrypted nonce
		nonce: String,
	},
}

#[derive(Deserialize, Debug)]
#[serde(tag = "op", rename_all = "snake_case")]
pub enum RemoteAuthGatewayServerOpCode {
	/// Defines the heartbeat and timeout intervals
	Hello {
		/// The minimum interval (in milliseconds) the client should heartbeat at
		heartbeat_interval: u64,
		/// The lifespan of the remote auth session (in milliseconds) before the connection is closed, typically a few minutes
		timeout_ms: u64,
	},
	/// Requests a cryptographic proof of the handshake
	NonceProof {
		/// The base64-encoded nonce encrypted with the client's public key
		encrypted_nonce: String,
	},
	/// Acknowledges a received client heartbeat
	HeartbeatAck,
	/// Acknowledges a successful handshake
	PendingRemoteInit {
		/// The base64URL-encoded SHA-256 digest of the client's public key
		fingerprint: String,
	},
	/// Acknowledges that the mobile session successfully created
	PendingTicket {
		/// The base64-encoded user payload encrypted with the client's public key
		encrypted_user_payload: String,
	},
	/// Indicates that the mobile session was completed
	PendingLogin {
		/// The ticket that can be used to obtain a token
		ticket: String,
	},
	/// Indicates that the mobile session was canceled
	Cancel,
}

pub fn REMOTE_AUTH_QR_CODE_URL(fingerprint: &str) -> String {
	format!("https://discord.com/ra/{fingerprint}")
}
