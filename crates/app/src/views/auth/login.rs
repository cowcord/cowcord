use dioxus::prelude::*;
use discord_api::endpoints::auth::login::{LoginAccountRequest, LoginAccountResponse};

use crate::components::ui::{Button, ButtonVariant};
use crate::utils::token::save_token;

#[component]
pub fn Login() -> Element {
	let nav = use_navigator();

	let mut email = use_signal(String::new);
	let mut password = use_signal(String::new);
	let mut error = use_signal(|| None::<String>);
	let mut loading = use_signal(|| false);
	let mut mfa_method = use_signal(|| None::<String>);

	let on_submit = move |e: FormEvent| {
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
				form { class: "flex flex-col gap-y-3 bg-muted p-6 rounded-lg w-1/2", onsubmit: on_submit,
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
