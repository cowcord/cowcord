use dioxus::prelude::*;

use crate::utils::local_storage::get_value;

#[component]
pub fn FateDecider() -> Element {
	let navigator = use_navigator();

	let redirect = match get_value("token") {
		| Some(_token) => "/channels/@me",
		| None => "/login",
	};

	navigator.replace(redirect);

	rsx! {}
}
