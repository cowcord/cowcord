use dioxus::prelude::*;

use crate::utils::local_storage::LocalStorage;

#[component]
pub fn Home() -> Element {
	let nav = use_navigator();

	let ls = match LocalStorage::new() {
		| Ok(ls) => ls,
		| Err(e) => {
			return rsx! {
				p { "An unexpected error occured! {e:?}" }
			};
		},
	};

	let redirect = match ls.get_value("token") {
		| Some(_token) => "/channels/@me",
		| None => "/login",
	};

	nav.replace(redirect);

	rsx! {}
}
