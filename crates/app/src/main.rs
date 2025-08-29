use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;
use discord_types::api::websocket::GatewayRecieveEvent;
use gloo_timers::callback::Interval;
use serde_json::json;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use web_sys::{BinaryType, MessageEvent, WebSocket, console};

pub mod utils;

mod views;
use views::*;

// pub mod components;
// use components::*;

// https://github.com/DioxusLabs/dioxus/issues/3211
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    // #[layout(ServerList)]
        // #[nest("/channels")]

    #[route("/")]
    Home {},

    #[route("/login")]
    Login {},

    #[route("/register")]
    Register {},

	#[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}

fn main() {
	dioxus::launch(App);
}

#[component]
fn App() -> Element {
	rsx! {
		document::Link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }

		Router::<Route> {}
	}
}
