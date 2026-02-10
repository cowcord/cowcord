use dioxus::prelude::*;

use crate::utils::local_storage::LocalStorage;

#[component]
pub fn Home() -> Element {
	let navigator = use_navigator();

	let redirect = match LocalStorage::new().map_err(|e| e).get_value("token") {
		| Some(_token) => "/channels/@me",
		| None => "/login",
	};

	navigator.replace(redirect);

	rsx! {}
}
