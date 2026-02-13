#![allow(non_snake_case)]

use dioxus::desktop::WindowBuilder;
use dioxus::prelude::*;

pub mod components;
pub mod utils;
pub mod ws;

mod views;
use views::*;

const TAILWIND: Asset = asset!("/assets/tailwind.css");

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::empty_line_after_outer_attr)]
enum Route {
    #[route("/")]
    Home {},

	#[route("/login")]
	Login {},

    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

fn main() {
	let config = dioxus::desktop::Config::new()
		.with_window(
			WindowBuilder::new()
				.with_maximized(true)
				.with_title("cowcord"), // .with_decorations(false), // .with_transparent(true),
		)
		.with_menu(None);

	// #[cfg(debug_assertions)]
	// config = config.with_disable_context_menu(true);

	LaunchBuilder::desktop().with_cfg(config).launch(|| {
		rsx! {
			Stylesheet { href: TAILWIND }
			Router::<Route> {}
		}
	});
}
