use dioxus::prelude::*;

mod views;
use views::{Home, Login};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[route("/login")]
    Login {},
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() { dioxus::launch(App); }

#[component]
fn App() -> Element {
	rsx! {
		document::Link { rel: "stylesheet", href: TAILWIND_CSS }

		Router::<Route> {}
	}
}
