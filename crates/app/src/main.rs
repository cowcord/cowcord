#![allow(non_snake_case)]

use dioxus::prelude::*;

pub mod utils;

mod views;
use views::*;

const TAILWIND: Asset = asset!("/assets/tailwind.css");

#[derive(Routable, Clone, Debug, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::empty_line_after_outer_attr)]
enum Route {
    #[route("/")]
    Home {},

    #[route("/:..route")]
    PageNotFound {
        route: Vec<String>,
    },
}

fn main() {
	dioxus::launch(|| {
		rsx! {
			Stylesheet { href: TAILWIND }
			Router::<Route> {}
		}
	});
}
