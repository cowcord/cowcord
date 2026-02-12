use base64::Engine;
use base64::prelude::{BASE64_STANDARD, BASE64_URL_SAFE_NO_PAD};
use dioxus::prelude::*;
use discord_api::REMOTE_AUTH_WS_URL;
use discord_api::endpoints::auth::login::{
	LoginAccountRequest,
	LoginAccountResponse,
	REMOTE_AUTH_TICKET_EXCHANGE,
	RemoteAuthTicketExchangeRequest,
	RemoteAuthTicketExchangeResponse,
};
use discord_api::types::ws::remote_auth::{
	REMOTE_AUTH_QR_CODE_URL,
	RemoteAuthGatewayClientOpCode,
	RemoteAuthGatewayServerOpCode,
};
use futures::{SinkExt, StreamExt};
use openssl::encrypt::Decrypter;
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private};
use openssl::rsa::{Padding, Rsa};
use openssl::sha::sha256;
use qrcode::render::svg;
use qrcode::{EcLevel, QrCode};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::time::{self, Duration, interval, timeout};
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::tungstenite::protocol::{CloseFrame, WebSocketConfig};
use tokio_tungstenite::tungstenite::{Error as WsError, Message};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

use crate::components::ui::{Button, ButtonVariant};
use crate::utils::request::RequestClient;
use crate::utils::token::save_token;
use crate::ws::remote_auth::RemoteAuthWsClient;

#[component]
pub fn Login() -> Element {
	let nav = use_navigator();

	let mut email = use_signal(String::new);
	let mut password = use_signal(String::new);
	let mut loading = use_signal(|| false);
	let mut qr_code_svg = use_signal(|| None::<String>);

	use_effect(move || {
		spawn(async move {
			match get_remote_auth_qr_url(qr_code_svg).await {
				| Ok(_) => {},
				| Err(e) => println!("remote auth error: {e}"),
			}
		});
	});

	let onsubmit = move |e: FormEvent| {
		e.prevent_default();

		spawn({
			let mut loading = loading.clone();
			let email = email.read().clone();
			let password = password.read().clone();
			let nav = nav.clone();

			async move {
				loading.set(true);

				match login_request(&email, &password).await {
					| Ok(resp) => {
						if let Some(token) = resp.token {
							save_token(&token.0);
							nav.replace("/channels/@me");
						}
					},
					| Err(err) => {
						println!("error: {err}");
					},
				}

				std::thread::sleep(std::time::Duration::from_secs(5));
				println!("heh");
				loading.set(false);
			}
		});
	};

	rsx! {
		div { class: "flex justify-center items-center h-screen flex-row",
			// div { class: "flex flex-row ",
				form { class: "flex flex-col gap-y-3 bg-muted p-6 rounded-lg w-1/2", onsubmit,
					h1 { class: "text-3xl text-center", "Welcome to Cowcord!" }
					div { class: "flex flex-col gap-y-1",
						p { "Email or Phone Number" }
						input {
							required: true,
							class: "border-border border bg-muted-darker rounded-md h-8",
							oninput: move |e| email.set(e.value()),
							"{email}"
						}
					}
					div { class: "flex flex-col gap-y-1",
						p { "Password" }
						input {
							r#type: "password",
							required: true,
							class: "border-border border bg-muted-darker rounded-md h-8",
							oninput: move |e| password.set(e.value()),
							"{password}"
						}
						a { class: "text-xs text-link", href: "", "Forgot your password?" }
					}
					Button {
						button_type: "submit",
						class: "py-2",
						disabled: loading(),
						"Log In"
					}
					div { class: "flex flex-row text-xs gap-x-1",
						p { "Need an account?" }
						a { class: "text-link", href: "register", "Register" }
					}
					if let Some(qr_code_svg) = qr_code_svg() {
						div { dangerous_inner_html: "{qr_code_svg}" }
					} else {
						p { "Loading QR code..." }
					}
				// }
			}
		}

	}
}

async fn login_request(
	email: &str,
	password: &str,
) -> Result<LoginAccountResponse, Box<dyn std::error::Error>> {
	let body = LoginAccountRequest {
		login: email.to_owned(),
		password: password.to_owned(),
		undelete: None,
		login_source: None,
		gift_code_sku_id: None,
	};

	Err("fuck you".into())
}

// todo: heartbeats, though it currently still works if youre fast enough
async fn get_remote_auth_qr_url(
	mut qr_code_svg: Signal<Option<String>>
) -> Result<(), Box<dyn std::error::Error>> {
	let mut client = RemoteAuthWsClient::connect().await?;

	// generate rsa and public key
	let rsa = Rsa::generate(2048)?;
	let public_key = rsa.public_key_to_der().unwrap();

	let pkey = PKey::from_rsa(rsa.clone())?;
	let mut decrypter = Decrypter::new(&pkey)?;
	decrypter.set_rsa_padding(Padding::PKCS1_OAEP)?;
	decrypter.set_rsa_oaep_md(MessageDigest::sha256())?;

	loop {
		let opcode = client
			.recv_json()
			.await?
			.ok_or("connection closed while waiting for mobile client")?;
		println!("recieved {:?}", &opcode);

		match opcode {
			// server sends tihs after we connect
			| RemoteAuthGatewayServerOpCode::Hello {
				heartbeat_interval,
				timeout_ms,
			} => {
				// we need to send Init opcode after recieving the Hello opcode
				client
					.send_json(&RemoteAuthGatewayClientOpCode::Init {
						encoded_public_key: BASE64_STANDARD.encode(&public_key),
					})
					.await?;
			},
			// if gateway accepts the public key we sent in the Init
			// then it will respond withan encrypted nonce
			// that we have to verify with our private key
			| RemoteAuthGatewayServerOpCode::NonceProof {
				encrypted_nonce,
			} => {
				// decrypt the nonce
				let encrypted_nonce_bytes = BASE64_STANDARD.decode(&encrypted_nonce)?;
				let buf_len = decrypter.decrypt_len(&encrypted_nonce_bytes)?;
				let mut decrypted_nonce = vec![0u8; buf_len];
				let len = decrypter.decrypt(&encrypted_nonce_bytes, &mut decrypted_nonce)?;
				let nonce_proof = BASE64_URL_SAFE_NO_PAD.encode(&decrypted_nonce[..len]);

				// send the decrypted nonce
				client
					.send_json(&RemoteAuthGatewayClientOpCode::NonceProof {
						nonce: nonce_proof,
					})
					.await?;
			},
			// server verifies we're waiting on the mobile device to init
			| RemoteAuthGatewayServerOpCode::PendingRemoteInit {
				fingerprint,
			} => {
				// validate the fingerprint we recieved
				let expected_fingerprint = BASE64_URL_SAFE_NO_PAD.encode(sha256(&public_key));
				let valid_fingerprint = fingerprint == expected_fingerprint;

				// if fingerprint isnt correct, close connection and reconnect
				if !valid_fingerprint {
					println!(
						"fingerprint mismatch! discord: {fingerprint} expected: {expected_fingerprint}"
					);
					client.close(None).await?;
					client = RemoteAuthWsClient::connect().await?;
				}

				// generate qr code from the fingerprint discord gave
				let url = REMOTE_AUTH_QR_CODE_URL(&fingerprint);
				let code = QrCode::with_error_correction_level(&url, EcLevel::L)?;
				let svg = code.render::<svg::Color>().min_dimensions(300, 300).build();
				qr_code_svg.set(Some(svg));
			},
			// mobile client has scanned the qr code
			| RemoteAuthGatewayServerOpCode::PendingTicket {
				encrypted_user_payload,
			} => {
				let encrypted_bytes = BASE64_STANDARD.decode(&encrypted_user_payload)?;
				let buf_len = decrypter.decrypt_len(&encrypted_bytes)?;
				let mut decrypted_payload = vec![0u8; buf_len];
				let len = decrypter.decrypt(&encrypted_bytes, &mut decrypted_payload)?;

				let decrypted_str = str::from_utf8(&decrypted_payload[..len])?;
				let mut parts = decrypted_str.split(':');

				// todo: update state ui or whatever from a qr code to like a check saying who youre logging in as
				let user_id = parts.next().unwrap();
				let discriminator = parts.next().unwrap();
				// 0 represents null avatar
				let avatar_hash = parts.next().unwrap();
				let username = parts.next().unwrap();

				println!("logged in as {username}#{discriminator}");
			},
			// mobile client has accepted the login attempt
			// and now we need to take the ticket and get our token
			| RemoteAuthGatewayServerOpCode::PendingLogin {
				ticket,
			} => {
				// create request client without sending an authorization header
				let http_client = RequestClient::new(true);

				// send ticket to ticket exchange endpoint
				let resp: RemoteAuthTicketExchangeResponse = http_client
					.post(
						REMOTE_AUTH_TICKET_EXCHANGE,
						Some(&RemoteAuthTicketExchangeRequest {
							ticket,
						}),
					)
					.await?;

				// decrypt response token
				let encrypted_bytes = BASE64_STANDARD.decode(&resp.encrypted_token)?;
				let buf_len = decrypter.decrypt_len(&encrypted_bytes)?;
				let mut decrypted_payload = vec![0u8; buf_len];
				let len = decrypter.decrypt(&encrypted_bytes, &mut decrypted_payload)?;
				let token = str::from_utf8(&decrypted_payload[..len])?;

				save_token(token)?;
				use_navigator().replace("/channels/@me");
			},
			| RemoteAuthGatewayServerOpCode::HeartbeatAck => {},
			| RemoteAuthGatewayServerOpCode::Cancel => {},
		}
	}

	Ok(())
}
