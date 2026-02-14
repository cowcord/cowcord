use dioxus::prelude::*;

use crate::utils::token::delete_token;

#[component]
pub fn Me() -> Element {
	let nav = use_navigator();
	rsx! {
		button {
			onclick: |_| {
				let _ = delete_token();
			},
			"delete token"
		}
		button {
			onclick: move |_| {
				nav.replace("/");
			},
			"goto home"
		}
	}
}
