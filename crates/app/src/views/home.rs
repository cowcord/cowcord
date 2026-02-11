use dioxus::prelude::*;

use crate::utils::token::load_token;

#[component]
pub fn Home() -> Element {
	let nav = use_navigator();

	let redirect = match load_token() {
		| Ok(Some(_token)) => "/channels/@me",
		| _ => "/login",
	};

	nav.replace(redirect);

	rsx! {}
}
