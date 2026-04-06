use std::sync::LazyLock;

use enigo::{Enigo, Keyboard, Settings};
use futures_util::lock::Mutex;
use log::{debug, warn};

use crate::{protobuf_message::pb};

static ENIGO:LazyLock<Mutex<Enigo>> = LazyLock::new(|| Mutex::new(Enigo::new(&Settings::default()).unwrap()));

pub async fn handle_presentation(pres: pb::Presentation) {
    match pres.actiontype {
        Some(pb::presentation::Actiontype::Prevslide(_)) => {
            ENIGO.lock().await.key(enigo::Key::LeftArrow, enigo::Direction::Click)
                .expect("Failed to send next previous key");
            debug!("Previous slide key sent");
        }

        Some(pb::presentation::Actiontype::Nextslide(_)) => {
            ENIGO.lock().await.key(enigo::Key::RightArrow, enigo::Direction::Click)
                .expect("Failed to send next slide key");
            debug!("Next slide key sent");
        }

        Some(pb::presentation::Actiontype::Present(_)) => {
            let mut enigo_unlocked = ENIGO.lock().await;
            enigo_unlocked.key(enigo::Key::Control, enigo::Direction::Press).unwrap();
            enigo_unlocked.key(enigo::Key::Alt, enigo::Direction::Press).unwrap();
            enigo_unlocked.key(enigo::Key::Unicode('p'), enigo::Direction::Press).unwrap();
            enigo_unlocked.key(enigo::Key::Control, enigo::Direction::Release).unwrap();
            enigo_unlocked.key(enigo::Key::Alt, enigo::Direction::Release).unwrap();
            enigo_unlocked.key(enigo::Key::Unicode('p'), enigo::Direction::Release).unwrap();
            debug!("Present slide key sent");
        }

        Some(pb::presentation::Actiontype::Fullscreen(_)) => {
            ENIGO.lock().await.key(enigo::Key::F11, enigo::Direction::Click)
                .expect("Failed to send fullscreen key");
            debug!("Fullscreen slide key sent");
        }

        Some(pb::presentation::Actiontype::Visibility(_)) => {
            ENIGO.lock().await.key(enigo::Key::Unicode('b'), enigo::Direction::Click)
                .expect("Failed to send visibility key");
            debug!("Slide visibility key sent");
        }

        None => {
            warn!("Empty presentation actiontype received");
        }
    }
}