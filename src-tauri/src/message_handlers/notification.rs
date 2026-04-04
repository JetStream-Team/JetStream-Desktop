use log::debug;
use lazy_static::lazy_static;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use std::process::Command;
use std::{collections::HashMap, fs, io::Write};
use tokio::sync::Mutex;

use crate::protobuf_message::pb;

#[cfg(any(target_os = "linux", target_os = "macos"))]
lazy_static! {
    // maps foreign android notification id to local dbus notification id
    static ref notif_store: Mutex<HashMap<u32, u32>> = Mutex::new(HashMap::new());
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub async fn handle_notification(notif: pb::Notification) {
    let mut notif_store_guard = notif_store.lock().await;

    if notif.create {
        debug!("--- Notification Received ---");
        debug!("Create: {}", notif.create);
        debug!("Id:     {}", notif.id);
        debug!("Title:  {}", notif.title);
        debug!("Body:   {}", notif.body);
        debug!("App:   {}", notif.app);

        let temp_icon_path = create_notif_icon(notif.id, &notif.icon);
        let notif_store_entry = notif_store_guard.get_mut(&notif.id);

        match notif_store_entry {
            // Notification needs to be updated
            Some(dbus_id) => {
                notify_rust::Notification::new()
                    .summary(&notif.title)
                    .body(&notif.body)
                    .appname("JetStream")
                    .icon(&temp_icon_path)
                    .hint(notify_rust::Hint::SuppressSound(true))
                    .action("open_device", "Open device")
                    .id(dbus_id.clone())
                    .show()
                    .expect("Failed to show updated notification");
                debug!("Notification updated");
            }

            // Notification needs to be created
            None => {
                let handle = notify_rust::Notification::new()
                    .summary(&notif.title)
                    .body(&notif.body)
                    .appname("JetStream")
                    .icon(&temp_icon_path)
                    .action("open_device", "Open device")
                    .show()
                    .expect("Failed to show notification");
                notif_store_guard.insert(notif.id, handle.id());

                tokio::task::spawn_blocking(move || {
                    handle.wait_for_action(|action| {
                        if action == "open_device" {
                            Command::new("scrcpy")
                            .arg(format!("--start-app={}", notif.app))
                            .spawn()
                            .ok();
                        }
                    });
                });

                debug!("Notification created");
            }
        }
    } else {
        debug!("Notification deletion event received: {}", notif.id);

        let notif_store_entry = notif_store_guard.remove(&notif.id);
        delete_notif_icon(notif.id);

        // Notification needs to be closed
        match notif_store_entry {
            Some(dbus_id) => {
                notify_rust::Notification::new()
                    .id(dbus_id)
                    .show()
                    .expect("Failed to get notification handle for closing")
                    .close();
                debug!("Notification closed");
            }
            None => {
                debug!("Notification not found in store");
            }
        }
    }
}

#[cfg(target_os = "windows")]
pub async fn handle_notification(notif: pb::Notification) {
    debug!("--- Notification Received ---");
    debug!("Create: {}", notif.create);
    debug!("Id:     {}", notif.id);
    debug!("Title:  {}", notif.title);
    debug!("Body:   {}", notif.body);

    let temp_icon_path = create_notif_icon(notif.id, &notif.icon);


    if notif.create {
        notify_rust::Notification::new()
            .summary(&notif.title)
            .body(&notif.body)
            .appname("JetStream")
            .icon(&temp_icon_path)
            .action("scrcpy", "Open device")
            .show()
            .expect("Failed to show notification");
    } else {
        debug!("Notification deletion event received: {}", notif.id);

        delete_notif_icon(notif.id);
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