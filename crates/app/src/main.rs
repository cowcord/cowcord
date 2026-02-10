use dioxus::prelude::*;

#[macro_use]
pub mod utils;

mod views;
use views::*;

const TAILWIND: Asset = asset!("/assets/tailwind.css");

fn main() {
	dioxus::launch(|| {
		rsx! {
			Stylesheet { href: TAILWIND }
			Router::<Route> {}
		}
	});
}

// Turn off rustfmt since we're doing layouts and routes in the same enum
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
