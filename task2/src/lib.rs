use futures_util::{future, pin_mut, SinkExt, StreamExt};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use web_sys::console;

#[wasm_bindgen]
pub fn ws_ping(endpoint: String, message: String) -> js_sys::Promise {
    future_to_promise(async move {
        console::log_1(&"Test".into());
        let ws = tokio_tungstenite_wasm::connect("ws://127.0.0.1:12345")
            .await
            .unwrap();
        let (mut sender, mut receiver) = ws.split();

        console::log_1(&"Connected to echo server.".into());

        sender
            .send(tokio_tungstenite_wasm::Message::text(message.as_str()))
            .await
            .unwrap();

        console::log_1(&"Sent payload to echo server. Waiting for response...".into());

        let msg = receiver.next().await.unwrap().unwrap();
        assert_eq!(msg, tokio_tungstenite_wasm::Message::text(message.as_str()));

        console::log_1(&"Received and validated response.".into());
        return Ok(JsValue::from_str(&msg.to_string()));
    })
}
