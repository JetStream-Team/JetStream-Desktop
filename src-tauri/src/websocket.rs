use log::{debug, error, info, trace};
use base64::{prelude::BASE64_STANDARD, Engine};
use futures_util::{SinkExt, StreamExt};
use local_ip_address::local_ip;
use core::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message as WSMessage;
use prost::Message;

use crate::{PORT, SERVER_NAME};
use crate::protobuf_message::handle_protobuf_message;

pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/jetstream.rs"));
}

pub async fn start_server(port: u16, app_handle: tauri::AppHandle) {
    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("Failed to bind to port {port}");
    info!("Listening on port 0.0.0.0:{port}");
    info!("Local IP address: {:?}", local_ip().expect("Failed to get local IP"));

    while let Ok((stream, addr)) = listener.accept().await {
        info!("New connection from {addr}");
        let app_handle = app_handle.clone();
        tokio::spawn(async move { handle_connection(stream, addr, app_handle).await });
    }
}

async fn handle_connection(stream: TcpStream, addr: SocketAddr, app_handle: tauri::AppHandle) {
    let ws_stream = accept_async(stream)
        .await
        .expect("Failed to accept WebSocket connection");
    info!("WebSocket connection accepted");

    if let Some(tray) = app_handle.tray_by_id("main") {
        let _ = tray.set_tooltip(Some("Connected"));
    }

    notify_rust::Notification::new()
        .summary("New device connected via JetStream")
        .body(&format!("IP: {addr}"))
        .show()
        .unwrap();

    let (mut outbox, mut inbox) = ws_stream.split();

    let identity_msg = pb::Identity {
        name: SERVER_NAME.to_string(),
        host: local_ip().unwrap().to_string(),
        port: PORT as u32,
    }.encode_to_vec();

    outbox.send(WSMessage::Binary(identity_msg.into()))
        .await.expect("Failed to send identity message");

    while let Some(result) = inbox.next().await {
        match result {
            Ok(ws_msg) => match ws_msg {
                WSMessage::Text(msg) => {
                    debug!("Received message: {msg}");
                    outbox.send(format!("Echo: {msg}").into()).await.unwrap();
                    let msg_bytes = BASE64_STANDARD
                        .decode(msg)
                        .expect("Failed to decode base64 message");
                    handle_protobuf_message(&msg_bytes).await;
                }
                WSMessage::Binary(msg) => {
                    trace!("Received binary message: {msg:?}");
                    handle_protobuf_message(&msg).await;
                }
                WSMessage::Close(frame) => {
                    let data = frame.clone().unwrap();
                    info!("Client disconnected: [{}] {}", data.code, data.reason);

                    if let Some(tray) = app_handle.tray_by_id("main") {
                        let _ = tray.set_tooltip(Some("Disconnected"));
                    }

                    notify_rust::Notification::new()
                        .summary("JetStream Device Disconnected")
                        .body(&format!("IP: {addr}"))
                        .show()
                        .unwrap();
                }
                _ => {}
            },
            Err(e) => {
                error!("Error receiving message: {e}");
                if let Some(tray) = app_handle.tray_by_id("main") {
                    let _ = tray.set_tooltip(Some("Disconnected"));
                }
                break;
            }
        };
    }
}