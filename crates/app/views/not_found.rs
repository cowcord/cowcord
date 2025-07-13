use dioxus::prelude::*;

#[component]
pub fn PageNotFound() -> Element {
	rsx! {
		p { "404" }
	}
}
