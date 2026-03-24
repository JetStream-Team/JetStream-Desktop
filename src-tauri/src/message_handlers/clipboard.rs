use log::debug;
use lazy_static::lazy_static;
use arboard::Clipboard;
use tokio::sync::Mutex;

use crate::protobuf_message::pb;

lazy_static! {
    static ref clipboard: Mutex<Clipboard> = Mutex::new(Clipboard::new().unwrap());
}

pub async fn handle_clipboard(cb: pb::Clipboard) {
    debug!("--- Clipboard Sync ---");
    debug!("Content: {}", cb.content);
    clipboard.lock().await.set_text(&cb.content).unwrap();
}