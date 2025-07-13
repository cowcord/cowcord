use js_sys::Reflect::{get, set};
use wasm_bindgen::JsValue;
use web_sys::window;

pub fn get_value(key: &str) -> Option<String> {
	if let Some(window) = window() {
		if let Ok(local_storage) = get(&window, &JsValue::from_str("localStorage")) {
			if let Ok(token_value) = get(&local_storage, &JsValue::from_str(key)) {
				if !token_value.is_undefined() && !token_value.is_null() {
					if let Some(token_str) = token_value.as_string() {
						if !token_str.is_empty() {
							return Some(token_str);
						}
					}
				}
			}
		}
	}
	None
}

pub fn save_value(
	key: &str,
	value: &str,
) {
	if let Some(window) = window() {
		if let Ok(local_storage) = get(&window, &JsValue::from_str("localStorage")) {
			let _ = set(
				&local_storage,
				&JsValue::from_str(key),
				&JsValue::from_str(value),
			);
		}
	}
}

pub fn remove_value(key: &str) {
	if let Some(window) = window() {
		if let Ok(local_storage) = get(&window, &JsValue::from_str("localStorage")) {
			let remove_item = JsValue::from_str("removeItem");
			let token_key = js_sys::Array::new();
			token_key.push(&JsValue::from_str(key));
			let _ = js_sys::Function::from(get(&local_storage, &remove_item).unwrap())
				.apply(&local_storage, &token_key);
		}
	}
}
