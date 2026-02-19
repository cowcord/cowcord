use std::time::Duration;

use base64::Engine;
use base64::prelude::{BASE64_STANDARD, BASE64_URL_SAFE_NO_PAD};
use dioxus::prelude::*;
use discord_api::endpoints::auth::login::{
	AuthenticatorType,
	LOGIN_ACCOUNT,
	LoginAccountRequest,
	LoginAccountResponse,
	LoginAccountSuspendedResponse,
	LoginRequiredActionType,
	REMOTE_AUTH_TICKET_EXCHANGE,
	RemoteAuthTicketExchangeRequest,
	RemoteAuthTicketExchangeResponse,
	VERIFY_MFA_LOGIN,
	VerifyMfaLoginRequest,
	VerifyMfaLoginResponse,
};
use discord_api::endpoints::cdn::USER_AVATAR;
use discord_api::types::token::Token;
use discord_api::types::ws::remote_auth::{
	REMOTE_AUTH_QR_CODE_URL,
	RemoteAuthGatewayClientOpCode,
	RemoteAuthGatewayServerOpCode,
};
use discord_api::{ApiResponse, CDN_URL};
use fast_qr::convert::Builder;
use fast_qr::convert::svg::SvgBuilder;
use fast_qr::{ECL, QRBuilder};
use lucide_dioxus::LoaderCircle;
use openssl::encrypt::Decrypter;
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::{Padding, Rsa};
use openssl::sha::sha256;
use tokio::time::{Instant, Interval, interval_at};

use crate::components::ui::Button;
use crate::utils::request::{AutoHandle, BaseUrl, RequestClient};
use crate::utils::token::save_token;
use crate::ws::remote_auth::RemoteAuthWsClient;

#[component]
pub fn Login() -> Element {
	let nav = use_navigator();

	let mut email = use_signal(String::new);
	let mut password = use_signal(String::new);
	let remote_auth_state = use_signal(|| RemoteAuthState::Loading);
	let password_login_state = use_signal(|| PasswordLoginState::Loading);
	let mut mfa_code = use_signal(String::new);

	use_resource(move || async move {
		trace!("Remote auth login state: {remote_auth_state:?}");
	});

	use_effect(move || {
		spawn(async move {
			match get_remote_auth_qr_url(remote_auth_state).await {
				| Ok(_) => {
					nav.replace("/channels/@me");
				},
				| Err(e) => error!("remote auth error: {e}"),
			}
		});
	});

	let password_onsubmit = move |e: FormEvent| {
		e.prevent_default();

		spawn({
			let email = email.read().clone();
			let password = password.read().clone();
			let nav = nav.clone();

			async move {
				match password_login(
					&LoginAccountRequest {
						login: email,
						password,
						undelete: None,
						login_source: None,
						gift_code_sku_id: None,
					},
					password_login_state,
				)
				.await
				{
					| Ok(_) => {
						if let PasswordLoginState::NeedMfa {
							authn_type: _,
							response: _,
						} = &*password_login_state.read()
						{
						} else {
							nav.replace("/channels/@me");
						}
					},
					| Err(err) => {
						error!("login error: {err}");
					},
				}
			}
		});
	};

	let mfa_onsubmit = move |e: FormEvent| {
		e.prevent_default();

		spawn({
			let mfa_code = mfa_code.read().clone();
			let password = password.read().clone();
			let nav = nav.clone();

			async move {
				if let PasswordLoginState::NeedMfa {
					authn_type,
					response,
				} = password_login_state()
				{
					match mfa_verify(&authn_type, response, mfa_code, password).await {
						| Ok(_) => {
							nav.replace("/channels/@me");
						},
						| Err(err) => {
							error!("mfa verify error: {err}");
						},
					}
				}
			}
		});
	};

	let qrcode_component = match &*remote_auth_state.read() {
		| RemoteAuthState::Loading => rsx! {
			PreAccepted {
				LoaderCircle { class: "animate-spin size-8 my-16" }
			}
		},
		| RemoteAuthState::QrCode {
			svg,
		} => {
			let src = format!(
				"data:image/svg+xml;base64,{}",
				BASE64_STANDARD.encode(svg.as_bytes())
			);
			rsx! {
				PreAccepted {
					img {
						class: "rounded-2xl",
						src: "{src}",
						width: "160",
						height: "160",
					}
				}
			}
		},
		| RemoteAuthState::Accepted {
			user_id,
			_discriminator,
			avatar_hash,
			username,
		} => {
			let avatar_url = format!("{}{}", CDN_URL, USER_AVATAR(user_id, avatar_hash));
			rsx! {
				div { class: "flex flex-col items-center gap-y-2 text-center",
					img { class: "rounded-full mb-4", src: "{avatar_url}" }
					h2 { class: "text-xl", "Check your phone!" }
					p { class: "text-sm text-muted-foreground", "Logging in as {username}" }
					a { class: "text-link text-xs", href: "", "Not me, start over" }
				}
			}
		},
		// discord client just restarts qr code process i think
		| RemoteAuthState::Cancelled => rsx! {
			p { "request cancelled by mobile client" }
		},
	};

	rsx! {
		div { class: "flex justify-center items-center h-screen flex-row",
			div { class: "flex flex-row bg-muted p-6 gap-x-10 rounded-lg w-1/2 max-w-180 min-h-1/3",
				if let PasswordLoginState::NeedMfa { authn_type, response: _ } = &*password_login_state
					.read()
				{
					form { class: "flex flex-col gap-y-3", onsubmit: mfa_onsubmit,
						p { "Enter your {authn_type} code" }
						input {
							required: true,
							class: "border-border border bg-muted-darker rounded-md h-8",
							oninput: move |e| mfa_code.set(e.value()),
						}
						Button { button_type: "submit", "Verify" }
					}
					a { class: "text-xs text-link", "Choose another MFA method" }
				} else {
					form { class: "flex flex-col gap-y-3 w-2/3", onsubmit: password_onsubmit,
						h2 { class: "text-2xl font-bold text-center mb-4", "Welcome to Cowcord!" }
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
						Button { button_type: "submit", class: "py-2",
							// disabled: loading(),
							"Log In"
						}
						div { class: "flex flex-row text-xs gap-x-1",
							p { "Need an account?" }
							a { class: "text-link", href: "/register", "Register" }
						}
					}
					div { class: "justify-center items-center flex m-auto", {qrcode_component} }
				}
			}
		}
	}
}

#[component]
fn PreAccepted(children: Element) -> Element {
	rsx! {
		div { class: "flex flex-col items-center gap-y-4 text-center",
			{children}
			h3 { class: "text-xl font-bold", "Log in with QR Code" }
			p { class: "text-sm text-muted-foreground",
				"Scan this with your Discord mobile app to log in"
			}
			a { class: "text-link text-xs", href: "", "or sign in with passkey" }
		}
	}
}

#[derive(Debug, Clone)]
enum PasswordLoginState {
	Loading,
	/// awaiting mfa verification
	NeedMfa {
		authn_type: AuthenticatorType,
		response: LoginAccountResponse,
	},
	/// account is suspended
	AccountSuspended {
		suspended_token: Token,
	},
	/// account marked for deletion, need to rerun with `undelete` set to true
	AccountDeleted,
	/// account was disabled, need to rerun with `undelete` set to true
	AccountDisabled,
	Success {
		required_actions: Vec<LoginRequiredActionType>,
	},
}

async fn password_login(
	payload: &LoginAccountRequest,
	mut password_login_state: Signal<PasswordLoginState>,
) -> Result<(), Box<dyn std::error::Error>> {
	trace!("Attempting login via email and password");
	let client = RequestClient::new(BaseUrl::Discord, true);

	// todo: new location stuff

	let resp = client
		.post::<LoginAccountRequest>(LOGIN_ACCOUNT, Some(payload))
		.await?;
	let status = resp.status();
	let body = resp.text().await?;

	if status.as_u16() == 403 {
		trace!("Account is suspended, getting suspended token");
		match serde_json::from_str::<LoginAccountSuspendedResponse>(&body) {
			| Ok(d) => {
				trace!("User account is suspended");
				password_login_state.set(PasswordLoginState::AccountSuspended {
					suspended_token: d.suspended_user_token,
				})
				// todo: handle suspended account
			},
			| Err(e) => return Err(e.into()),
		};
	}

	match serde_json::from_str::<ApiResponse<LoginAccountResponse>>(&body)? {
		| ApiResponse::Success(r) => {
			if r.mfa.unwrap_or_default() {
				trace!("MFA required for login");
				let authn_type = if r.totp.unwrap_or_default() {
					AuthenticatorType::totp
				} else if r.backup.unwrap_or_default() {
					AuthenticatorType::backup
				} else if r.sms.unwrap_or_default() {
					// todo: send mfa code to sms via endpoint
					AuthenticatorType::sms
				} else if let Some(Some(credential_request_options)) = &r.webauthn {
					AuthenticatorType::webauthn {
						credential_request_options: credential_request_options.to_owned(),
					}
				} else {
					AuthenticatorType::password
				};

				password_login_state.set(PasswordLoginState::NeedMfa {
					authn_type,
					response: r,
				});

				return Ok(());
			}

			// can probably unwrap here cus if token is not provided
			// then the mfa and suspended token thing should catch that
			// and return before this is called
			let _ = save_token(&r.token.unwrap().0);
			password_login_state.set(PasswordLoginState::Success {
				required_actions: r.required_actions.unwrap_or_default(),
			});
		},
		| ApiResponse::Error(e) => {
			match e.code {
				| 70007 => {
					// password_login_state.set(PasswordLoginState::);
					// phone verification
				},
				| 20013 => {
					password_login_state.set(PasswordLoginState::AccountDisabled);
					// account disabled, redo request with `undelete` set to true to enable account
				},
				| 20011 => {
					password_login_state.set(PasswordLoginState::AccountDeleted);
					// account marked for deletion, redo request with `undelete` set to true to enable account
				},
				| _ => {},
			}

			return Err(format!("{:?}", e).into());
		},
	}

	Ok(())
}

async fn mfa_verify(
	authn_type: &AuthenticatorType,
	login_response: LoginAccountResponse,
	code: String,
	password: String,
) -> Result<(), Box<dyn std::error::Error>> {
	let client = RequestClient::new(BaseUrl::Discord, true);

	let code = match authn_type {
		| AuthenticatorType::webauthn {
			credential_request_options: _,
		} => return Err("webauthn is not yet supported".into()),
		| AuthenticatorType::password => password,
		| _ => code,
	};

	let payload = VerifyMfaLoginRequest {
		code,
		ticket: login_response.ticket.unwrap_or_default(),
		login_source: Some(None),
		gift_code_sku_id: Some(None),
		login_instance_id: login_response.login_instance_id,
	};

	println!("{}", serde_json::to_string(&payload)?);

	let resp: ApiResponse<VerifyMfaLoginResponse> = client
		.post::<VerifyMfaLoginRequest>(&VERIFY_MFA_LOGIN(authn_type), Some(&payload))
		.await?
		.with_auto_handle()
		.await?;

	return match resp {
		| ApiResponse::Success(r) => {
			let _ = save_token(&r.token);
			Ok(())
		},
		| ApiResponse::Error(e) => Err(format!("{e:?}").into()),
	};
}

#[derive(Debug)]
enum RemoteAuthState {
	/// fingerprint has not yet been recieved
	Loading,
	/// fingerprint has been recieved and the qr code has been saved
	QrCode {
		svg: String,
	},
	/// mobile client has accepted the connection
	Accepted {
		user_id: String,
		_discriminator: String,
		avatar_hash: String,
		username: String,
	},
	Cancelled,
}

async fn get_remote_auth_qr_url(
	mut remote_auth_state: Signal<RemoteAuthState>
) -> Result<(), Box<dyn std::error::Error>> {
	'reconnect: loop {
		let mut client = RemoteAuthWsClient::connect().await?;

		// generate rsa and public key
		let rsa = Rsa::generate(2048)?;
		let public_key = rsa.public_key_to_der().unwrap();

		let pkey = PKey::from_rsa(rsa.clone())?;
		let mut decrypter = Decrypter::new(&pkey)?;
		decrypter.set_rsa_padding(Padding::PKCS1_OAEP)?;
		decrypter.set_rsa_oaep_md(MessageDigest::sha256())?;

		let mut heartbeat_interval: Option<Interval> = None;
		let mut awaiting_ack = false;

		loop {
			let hb_tick = async {
				match heartbeat_interval.as_mut() {
					| Some(iv) => {
						iv.tick().await;
					},
					| None => std::future::pending().await,
				}
			};

			tokio::select! {
				_ = hb_tick => {
					if awaiting_ack {
						warn!("heartbeat ack expected but not recieved, reconnecting...");
						let _ = client.close(None).await;
						continue 'reconnect;
					}
					client.send_json(&RemoteAuthGatewayClientOpCode::Heartbeat).await?;
					awaiting_ack = true;
				}

				result = client.recv_json() => {
					let opcode = result?
						.ok_or("connection closed while waiting for mobile client")?;
					trace!("remote auth ws recieved {:?}", &opcode);

					// if awaiting_ack {
					// 	if !matches!(opcode, RemoteAuthGatewayServerOpCode::HeartbeatAck) {
					// 		let _ = client.close(None).await;
					// 		continue 'reconnect;
					// 	}
					// 	awaiting_ack = false;
					// 	continue;
					// }

					match opcode {
						// server sends tihs after we connect
						| RemoteAuthGatewayServerOpCode::Hello {
							heartbeat_interval: interval_ms,
							timeout_ms: _,
						} => {
							let start = Instant::now() + Duration::from_millis(interval_ms);
							heartbeat_interval = Some(interval_at(start, Duration::from_millis(interval_ms)));

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
								error!(
									"fingerprint mismatch! discord: {fingerprint} expected: {expected_fingerprint}, reconnecting..."
								);
								let _ = client.close(None).await;
								continue 'reconnect;
							}

							// generate qr code from the fingerprint discord gave
							let url = REMOTE_AUTH_QR_CODE_URL(&fingerprint);
							let qr = QRBuilder::new(url).ecl(ECL::L).build().unwrap();
							let svg = SvgBuilder::default().margin(2).to_str(&qr);

							remote_auth_state.set(RemoteAuthState::QrCode {
								svg,
							});
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

							remote_auth_state.set(RemoteAuthState::Accepted {
								user_id: parts.next().unwrap().to_owned(),
								_discriminator: parts.next().unwrap().to_owned(),
								avatar_hash: parts.next().unwrap().to_owned(),
								username: parts.next().unwrap().to_owned(),
							});
						},
						// mobile client has accepted the login attempt
						// and now we need to take the ticket and get our token
						| RemoteAuthGatewayServerOpCode::PendingLogin {
							ticket,
						} => {
							// create request client without sending an authorization header
							let http_client = RequestClient::new(BaseUrl::Discord, true);

							// send ticket to ticket exchange endpoint
							let resp: ApiResponse<RemoteAuthTicketExchangeResponse> = http_client
								.post(
									REMOTE_AUTH_TICKET_EXCHANGE,
									Some(&RemoteAuthTicketExchangeRequest {
										ticket,
									}),
								)
								.await?
								.with_auto_handle()
								.await?;

							return match resp {
								ApiResponse::Success(v) => {
									let encrypted_bytes=BASE64_STANDARD.decode(&v.encrypted_token)?;
									let buf_len=decrypter.decrypt_len(&encrypted_bytes)?;
									let mut decrypted_payload=vec![0u8;buf_len];
									let len=decrypter.decrypt(&encrypted_bytes, &mut decrypted_payload)?;
									let token=str::from_utf8(&decrypted_payload[..len])?;

									save_token(token)?;
									let _ = client.close(None).await;
									Ok(())
								},
								ApiResponse::Error(e) => Err(format!("{:?}", e).into()),
							}
						},
						| RemoteAuthGatewayServerOpCode::HeartbeatAck => {
							awaiting_ack = false;
						},
						| RemoteAuthGatewayServerOpCode::Cancel => {
							remote_auth_state.set(RemoteAuthState::Cancelled);
						},
					}
				}
			}
		}
	}
}
