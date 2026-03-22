use std::{fs, io::Write};

use arboard::Clipboard;
use log::{debug, error, warn};
use prost::Message;
use tokio::sync::Mutex;
use lazy_static::lazy_static;


pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/jetstream.rs"));
}

lazy_static! {
    static ref clipboard: Mutex<Clipboard> = Mutex::new(Clipboard::new().unwrap());
}

pub async fn handle_protobuf_message(data: &[u8]) {
    match pb::MessageWrapper::decode(data) {
        Ok(wrapper) => {

            debug!("Decoded wrapped protobuf message: {wrapper:?}");

            match wrapper.message {

                // Notification Message
                Some(pb::message_wrapper::Message::Notification(notif)) => {
                    debug!("--- Notification Received ---");
                    debug!("Create: {}", notif.create);
                    debug!("Id:     {}", notif.id);
                    debug!("Title:  {}", notif.title);
                    debug!("Body:   {}", notif.body);

                    let temp_icon_path = std::env::temp_dir()
                        .join(format!("jetstream_icon_{}.png", notif.id))
                        .to_string_lossy()
                        .to_string();
                    let mut temp_icon_file = fs::File::create(&temp_icon_path)
                        .expect("Failed to create notification icon file");
                    temp_icon_file.write_all(&notif.icon)
                        .expect("Failed to write notification icon to file");
                    temp_icon_file.flush()
                        .expect("Failed to flush notification icon file");

                    if notif.create {
                        notify_rust::Notification::new()
                            .summary(&notif.title)
                            .body(&notif.body)
                            .appname("JetStream")
                            .icon(&temp_icon_path)
                            .show()
                            .expect("Failed to show notification");
                    } else {
                        debug!("Notification deletion event received: {}", notif.id);

                        let temp_icon_path = std::env::temp_dir()
                            .join(format!("jetstream_icon_{}.png", notif.id));
                        if temp_icon_path.exists() {
                            fs::remove_file(temp_icon_path)
                                .expect("Failed to remove notification icon file");
                        }
                    }
                }

                // Clipboard Message
                Some(pb::message_wrapper::Message::Clipboard(cb)) => {
                    debug!("--- Clipboard Sync ---");
                    debug!("Content: {}", cb.content);
                    clipboard.lock().await.set_text(&cb.content).unwrap();
                }

                // Unknown Message
                None => {
                    warn!("Received an empty or unknown protobuf Message");
                }
            }
        }

        Err(e) => {
            error!("Failed to decode protobuf message: {e}");
        }
    }
}
