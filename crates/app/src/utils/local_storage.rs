use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsError, JsValue};
use web_sys::Storage;

#[wasm_bindgen]
pub struct LocalStorage(Storage);

#[wasm_bindgen]
impl LocalStorage {
	#[wasm_bindgen(constructor)]
	#[must_use]
	pub fn new() -> Result<LocalStorage, JsError> {
		let window = web_sys::window().ok_or(JsError::new(""))?;
		let storage = window
			.local_storage()
			.unwrap_or(None)
			.ok_or(JsError::new(""))?;

		Ok(LocalStorage(storage))
	}

	#[wasm_bindgen]
	pub fn get_value(
		&self,
		key: &str,
	) -> Option<String> {
		self.0.get_item(key).unwrap_or(Some("Unknown".to_string()))
	}

	#[wasm_bindgen]
	pub fn set_value(
		&self,
		key: &str,
		value: &str,
	) -> Result<(), JsValue> {
		self.0.set_item(key, value)
	}

	#[wasm_bindgen]
	pub fn remove_value(
		&self,
		key: &str,
	) -> Result<(), JsValue> {
		self.0.remove_item(key)
	}
}
