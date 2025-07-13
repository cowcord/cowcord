use std::error::Error;

use dioxus::prelude::*;
use web_sys::console;

use crate::models::endpoints::auth::{LoginRequest, LoginResponse /* , MfaRequest */};
use crate::utils::local_storage::save_value;
use crate::utils::request::RequestClient;

async fn login(info: LoginRequest) -> Result<LoginResponse, Box<dyn Error>> {
	let client = RequestClient::new();

	let response: LoginResponse = client.post("/auth/login", Some(&info)).await?;

	if let Some(token) = &response.token {
		save_value("token", token);
	}
	Ok(response)
}

#[component]
pub fn Login() -> Element {
	// let mut show_modal = use_signal(|| false);
	// let mut ticket = use_signal(|| None);

	rsx! {
		form {
			onsubmit: move |event: Event<FormData>| {
				spawn(async move {
					let identifier = event
						.values()
						.get("identifier")
						.and_then(|val| val.get(0).cloned())
						.unwrap_or_default();
					let password = event
						.values()
						.get("password")
						.and_then(|val| val.get(0).cloned())
						.unwrap_or_default();
					let request = LoginRequest {
						login: identifier,
						password,
						undelete: None,
						login_source: None,
					};
					match login(request).await {
						Ok(_login_response) => {
							// if login_response.mfa.unwrap_or(false) {
							// 	if let Some(ticket_value) = login_response.ticket {
							// 		let sms_request = SmsMfaRequest {
							// 			token: ticket_value.clone(),
							// 		};
							// 		match send_sms_mfa(sms_request).await {
							// 			Ok(_) => {
							// 				ticket.set(Some(ticket_value));
							// 				show_modal.set(true);
							// 			}
							// 			Err(e) => {
							// 				console::error_1(
							// 					&format!("Error sending sms MFA code: {}", e).into(),
							// 				)
							// 			}
							// 		}
							// 	} else {
							// 		console::error_1(
							// 			&"MFA required, but no ticket received!".into(),
							// 		);
							// 	}
							// }
						}
						Err(e) => console::error_1(&format!("Login failed: {}", e).into()),
					}
				});
			},
			input { name: "identifier", placeholder: "Email or Phone number" }
			input { name: "password", placeholder: "Password" }
			input { r#type: "submit", value: "Login" }
		}

		// if show_modal() {
		// 	Modal {
		// 		on_submit: move |code: String| {
		// 		    show_modal.set(false);
		// 		    if let Some(ticket_value) = ticket() {
		// 		        let mfa_request = MfaRequest {
		// 		            ticket: ticket_value,
		// 		            code,
		// 		            login_source: None,
		// 		        };
		// 		        spawn(async move {
		// 		            match mfa_login(mfa_request).await {
		// 		                Ok(_login_response) => {
		// 		                    ticket.set(None);
		// 		                    show_modal.set(false);
		// 		                }
		// 		                Err(e) => {
		// 		                    console::error_1(&format!("MFA Login failed: {}", e).into());
		// 		                }
		// 		            }
		// 		        });
		// 		    } else {
		// 		        console::error_1(&"No ticket found for MFA submission!".into());
		// 		    }
		// 		},
		// 	}
		// }
	}
}
