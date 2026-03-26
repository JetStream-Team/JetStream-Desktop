use log::{debug, error};
use std::process::Command;

use crate::protobuf_message::pb;

pub async fn handle_action(action: pb::Action) {
    match action.actiontype {
        Some(pb::action::Actiontype::Lock(_)) => {
            debug!("Received lock action");

            #[cfg(target_os = "windows")]
            Command::new("rundll32.exe")
                .args(["user32.dll,LockWorkStation"])
                .spawn()
                .unwrap();

            #[cfg(target_os = "linux")]
            Command::new("loginctl")
                .arg("lock-session")
                .spawn()
                .unwrap();

            #[cfg(target_os = "macos")]
            error!("Lock action not implemented on macOS");
        }
        Some(pb::action::Actiontype::Poweroff(_)) => {
            debug!("Received poweroff action");

            #[cfg(target_os = "windows")]
            Command::new("shutdown")
                .arg("/s")
                .arg("/t")
                .arg("0")
                .spawn()
                .unwrap();

            #[cfg(target_os = "linux")]
            Command::new("systemctl")
                .arg("poweroff")
                .spawn()
                .unwrap();

            error!("Poweroff action not implemented on macOS");
        }
        Some(pb::action::Actiontype::Reboot(_)) => {
            debug!("Received reboot action");

            #[cfg(target_os = "windows")]
            Command::new("shutdown")
                .arg("/r")
                .arg("/t")
                .arg("0")
                .spawn()
                .unwrap();

            #[cfg(target_os = "linux")]
            Command::new("systemctl")
                .arg("reboot")
                .spawn()
                .unwrap();

            #[cfg(target_os = "macos")]
            error!("Reboot action not implemented on macOS");
        }
        None => {
            debug!("Received unknown or empty action");
        }
    }
}