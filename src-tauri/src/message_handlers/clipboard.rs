use log::debug;
use std::sync::LazyLock;
use arboard::Clipboard;
use prost::Message;
use tokio::sync::Mutex;

use crate::{protobuf_message::pb, websocket::OUTBOX};

static CLIPBOARD: LazyLock<Mutex<Clipboard>> = LazyLock::new(||Mutex::new(Clipboard::new().unwrap()));

pub async fn handle_clipboard(cb: pb::Clipboard) {
    debug!("--- Clipboard Sync ---");
    debug!("Content: {}", cb.content);
    CLIPBOARD.lock().await.set_text(&cb.content).unwrap();
}

pub fn send_clipboard() {
    tokio::spawn(async {

        if !OUTBOX.is_connected().await { return; }

        let text = CLIPBOARD.lock().await
          .get_text().expect("Failed to get text from clipboard");
        let wrapper = pb::MessageWrapper {
            message: Some(pb::message_wrapper::Message::Clipboard(
                pb::Clipboard { content: text }
            ))
        };

        let encoded = wrapper.encode_to_vec();
        OUTBOX.send(encoded.into()).await.expect("Failed to send clipboard");
        debug!("Set clipboard");
    });
}