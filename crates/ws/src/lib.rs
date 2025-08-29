use wasm_bindgen::prelude::*;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};
use discord_types::{API_VERSION, api::endpoints::websocket::{WEBSOCKET, WebSocketQueryParams}, api::endpoints:websocket::GatewayEncoding};

const query: WebSocketQueryParams {
    v: API_VERSION::v9
    encoding: GatewayEncoding::json,
    compress: None,
}

#[wasm_bindgen(start)]
fn start_websocket() -> Result<(), JsValue> {
    let ws = WebSocket::new(WEBSOCKET(query))?;
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
    let cloned_ws = ws.clone();
    let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
            // binary message
            let array = js_sys::Uint8Array::new(&abuf);
            let len = array.byte_length() as usize;
            cloned_ws.set_binary_type(web_sys::BinaryType::Blob);
            match cloned_ws.send_with_u8_array(&[5, 6, 7, 8]) {
                Ok(_) => {},
                Err(err) => {},
            }
        } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
            let fr = web_sys::FileReader::new().unwrap();
            let fr_c = fr.clone();
            let onloadend_cb = Closure::<dyn FnMut(_)>::new(move |_e: web_sys::ProgressEvent| {
                let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
                let len = array.byte_length() as usize;
                // image
            });
            fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
            fr.read_as_array_buffer(&blob).expect("blob not readable");
            onloadend_cb.forget();
        } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
            // text
        } else {
        }
    });
    ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    let onerror_callback = Closure::<dyn FnMut(_)>::new(move |e: ErrorEvent| {
    });
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let cloned_ws = ws.clone();
    let onopen_callback = Closure::<dyn FnMut()>::new(move || {
        match cloned_ws.send_with_str("ping") {
            Ok(_) => {},
            Err(err) => {},
        }
        // send binary message
        match cloned_ws.send_with_u8_array(&[0, 1, 2, 3]) {
            Ok(_) => {},
            Err(err) => {},
        }
    });
    ws.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    Ok(())
}
