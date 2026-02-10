use dioxus::prelude::*;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
	rsx! {
		p { "404" }
	}
}
