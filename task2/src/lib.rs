use futures_util::{SinkExt, StreamExt};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::console;

#[wasm_bindgen]
pub fn ws_ping(endpoint: String, message: String) -> js_sys::Promise {
    future_to_promise(async move {
        console::log_1(&"Trying to make a connection".into());
        let ws = tokio_tungstenite_wasm::connect(endpoint)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let (mut sender, mut receiver) = ws.split();

        console::log_1(&"Connected to echo server.".into());

        sender
            .send(tokio_tungstenite_wasm::Message::text(message.as_str()))
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        console::log_1(&"Sent payload to echo server. Waiting for response...".into());

        let msg = receiver
            .next()
            .await
            .ok_or(JsValue::from_str("No data"))?
            .map_err(|e| JsValue::from_str(&format!("Error while receiving data: {e}")))?;

        console::log_1(&"Received and validated response.".into());

        Ok(JsValue::from_str(&msg.to_string()))
    })
}
