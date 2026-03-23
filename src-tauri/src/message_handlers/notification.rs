use log::debug;
use lazy_static::lazy_static;
use notify_rust::NotificationHandle;
use std::{collections::HashMap, fs, io::Write};
use tokio::sync::Mutex;

use crate::protobuf_message::pb;

lazy_static! {
    static ref notif_store: Mutex<HashMap<u32, NotificationHandle>> = Mutex::new(HashMap::new());
}

pub async fn handle_notification(notif: pb::Notification) {
    let mut notif_store_guard = notif_store.lock().await;

    if notif.create {
        debug!("--- Notification Received ---");
        debug!("Create: {}", notif.create);
        debug!("Id:     {}", notif.id);
        debug!("Title:  {}", notif.title);
        debug!("Body:   {}", notif.body);

        let temp_icon_path = create_notif_icon(notif.id, &notif.icon);
        let notif_store_entry = notif_store_guard.get_mut(&notif.id);

        match notif_store_entry {
            // Notification needs to be updated
            Some(handle) => {
                handle
                    .summary(&notif.title)
                    .body(&notif.body)
                    .appname("JetStream")
                    .icon(&temp_icon_path)
                    .hint(notify_rust::Hint::SuppressSound(true))
                    .action("scrcpy", "Open device");
                handle.update();
                debug!("Notification updated");
            }

            // Notification needs to be created
            None => {
                let handle = notify_rust::Notification::new()
                    .summary(&notif.title)
                    .body(&notif.body)
                    .appname("JetStream")
                    .icon(&temp_icon_path)
                    .action("scrcpy", "Open device")
                    .show()
                    .expect("Failed to show notification");
                notif_store_guard.insert(notif.id, handle);
                debug!("Notification created");
            }
        }
    } else {
        debug!("Notification deletion event received: {}", notif.id);

        let notif_store_entry = notif_store_guard.remove(&notif.id);
        delete_notif_icon(notif.id);

        // Notification needs to be closed
        match notif_store_entry {
            Some(handle) => {
                handle.close();
                debug!("Notification closed");
            }
            None => {
                debug!("Notification not found in store");
            }
        }
    }
}


fn create_notif_icon(id: u32, bytes: &[u8]) -> String {
    let temp_icon_path = std::env::temp_dir()
        .join(format!("jetstream_icon_{}.png", id))
        .to_string_lossy()
        .to_string();
    let mut temp_icon_file = fs::File::create(&temp_icon_path)
        .expect("Failed to create notification icon file");
    temp_icon_file.write_all(&bytes)
        .expect("Failed to write notification icon to file");
    temp_icon_file.flush()
        .expect("Failed to flush notification icon file");

    return temp_icon_path;
}

fn delete_notif_icon(id: u32) {
    let temp_icon_path = std::env::temp_dir()
        .join(format!("jetstream_icon_{}.png", id));
    if temp_icon_path.exists() {
        fs::remove_file(temp_icon_path)
            .expect("Failed to remove notification icon file");
    }
}