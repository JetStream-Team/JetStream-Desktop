use log::{error, trace, warn};
use prost::{Message};

use crate::{message_handlers::{actions, clipboard, notification, presentation}};

pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/jetstream.rs"));
}

pub async fn handle_protobuf_message(data: &[u8]) {
    match pb::MessageWrapper::decode(data) {
        Ok(wrapper) => {

            trace!("Decoded wrapped protobuf message: {wrapper:?}");

            match wrapper.message {
                // Identity Message
                Some(pb::message_wrapper::Message::Identity(_iden)) => {}

                // Open App Message
                Some(pb::message_wrapper::Message::Openapp(_app)) => {}

                // Notification Message
                Some(pb::message_wrapper::Message::Notification(notif)) => {
                    notification::handle_notification(notif).await;
                }

                // Clipboard Message
                Some(pb::message_wrapper::Message::Clipboard(cb)) => {
                    clipboard::handle_clipboard(cb).await;
                }

                // Action Message
                Some(pb::message_wrapper::Message::Action(action)) => {
                    actions::handle_action(action).await;
                }

                // Presentation Message
                Some(pb::message_wrapper::Message::Presentation(presentation)) => {
                    presentation::handle_presentation(presentation).await;
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