use std::cell::RefCell;
use std::rc::Rc;

use dioxus::prelude::*;
use gloo_timers::callback::Interval;
use models::http::websocket::GatewayRecieveEvent;
use serde_json::json;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use web_sys::{BinaryType, MessageEvent, WebSocket, console};

#[macro_use]
pub mod utils;

mod views;
use views::*;

// pub mod components;
// use components::*;

pub mod models;

// https://github.com/DioxusLabs/dioxus/issues/3211
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::empty_line_after_outer_attr)]
pub enum Route {
    #[layout(ServerList)]
        // #[nest("/channels")]
        // #[end_nest]
    #[end_layout]

    #[route("/")]
    FateDecider {},

    #[route("/login")]
    Login {},

    #[route("/register")]
    Register {},

	#[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}

const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
	dioxus::launch(App);
}

#[component]
fn App() -> Element {
	rsx! {
		document::Link { rel: "stylesheet", href: TAILWIND_CSS }

		Router::<Route> {}
	}
}

fn start_websocket(token: &str) {
	let gateway_url = "wss://gateway.discord.gg/?encoding=json&v=9";

	let ws = WebSocket::new(gateway_url).expect("Failed to create WebSocket");
	ws.set_binary_type(BinaryType::Arraybuffer);

	let sequence = Rc::new(RefCell::new(Option::<i64>::None));
	let heartbeat_interval = Rc::new(RefCell::new(None::<Interval>));

	let ws_clone = ws.clone();
	let token = token.to_string();

	let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
		if let Ok(text) = e.data().dyn_into::<js_sys::JsString>() {
			let text_str = text.as_string().unwrap();
			let data: GatewayRecieveEvent =
				serde_json::from_str(&text_str).expect("Failed to parse JSON");
			console::log_1(&format!("{}", &text_str).into());

			match data.op {
				| 0 => {
					// dispatch event
					handle_dispatch(data.clone())
				},
				| 7 => { // reconnect
				},
				| 9 => {
					// invalid session
					if data.d == true {
						// can reconnect
					}
				},
				| 10 => {
					// hello
					let interval_ms = data.d["heartbeat_interval"].as_f64().unwrap_or(41250.0);

					if let Some(interval) = heartbeat_interval.borrow_mut().take() {
						interval.cancel();
					}

					let ws_heartbeat = ws.clone();
					let seq_for_heartbeat = *sequence.borrow();
					let interval = Interval::new(interval_ms as u32, move || {
						let heartbeat = json!({
							"op": 1,
							"d": seq_for_heartbeat
						});
						let _ = ws_heartbeat.send_with_str(&heartbeat.to_string());
					});

					*heartbeat_interval.borrow_mut() = Some(interval);

					let identify = json!({
						"op": 2,
						"d": {
							"token": token,
							"intents": 3276799,
							"properties": {
								// TODO: get real properties
								"os": "Windows",
								"browser": "Chrome",
								"device": ""
							}
						}
					})
					.to_string();

					ws.clone()
						.send_with_str(&identify)
						.expect("Failed to send Identify");
				},
				| 11 => { // heartbeat ack
				},
				| _ => {
					// this should never happen unless discord adds something
					console::error_1(
						&format!("Unhandled response, please report this: {:#?}", data).into(),
					);
				},
			}

			if let Some(seq) = data.s {
				*sequence.borrow_mut() = Some(seq);
			}
		}
	}) as Box<dyn FnMut(MessageEvent)>);

	let onerror_callback = Closure::wrap(Box::new(move |e: web_sys::ErrorEvent| {
		console::error_1(&format!("Websocket error: {}", e.message()).into());
	}) as Box<dyn FnMut(web_sys::ErrorEvent)>);

	let onopen_callback = Closure::wrap(Box::new(move |_| {
		console::log_1(&"Websocket connection started".into());
	}) as Box<dyn FnMut(JsValue)>);

	let onclose_callback = Closure::wrap(Box::new(move |e: web_sys::CloseEvent| {
		console::log_1(&format!("Websocket closed: {} ({})", e.reason(), e.code()).into());
	}) as Box<dyn FnMut(web_sys::CloseEvent)>);

	ws_clone.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
	ws_clone.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
	ws_clone.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
	ws_clone.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));

	onmessage_callback.forget();
	onerror_callback.forget();
	onopen_callback.forget();
	onclose_callback.forget();
}

fn handle_dispatch(data: GatewayRecieveEvent) {
	if let Some(event) = data.t.as_deref() {
		match event {
			| "READY" => {
				// Self-explanatory
			},
			| _ => {},
		}
	}
}
