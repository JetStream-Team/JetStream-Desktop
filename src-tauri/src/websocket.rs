use log::{debug, error, info, trace};
// use std::collections::HashMap;
// use crate::certs;
// use rustls::ServerConfig;
// use tokio_rustls::{TlsAcceptor, server::TlsStream};
use base64::{prelude::BASE64_STANDARD, Engine};
use futures_util::{SinkExt, StreamExt};
use local_ip_address::local_ip;
use notify_rust;
use core::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::{Message as WSMessage};
use prost::Message;

#[allow(dead_code)]
pub type WSInbox = futures_util::stream::SplitStream<tokio_tungstenite::WebSocketStream<TcpStream>>;
#[allow(dead_code)]
pub type WSOutbox = futures_util::stream::SplitSink<
    tokio_tungstenite::WebSocketStream<TcpStream>,
    tokio_tungstenite::tungstenite::Message,
>;

use crate::{PORT, SERVER_NAME};
use crate::protobuf_message::handle_protobuf_message;

pub mod pb {
    include!(concat!(env!("OUT_DIR"), "/jetstream.rs"));
}

pub async fn start_server(port: u16) {
    // Removed because to tls
    // let (certs, key) = certs::get_cert();
    // println!("{certs}");
    // println!("{key}");

    // let config = ServerConfig::builder()
    //     .with_no_client_auth()
    //     .with_single_cert(certs, key)
    //     .expect("Failed to create TLS config");

    // let acceptor = TlsAcceptor::from(Arc::new(config));

    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("Failed to bind to port {port}");
    info!("Listening on port 0.0.0.0:{port}");
    info!("Local IP address: {:?}", local_ip().expect("Failed to get local IP"));

    while let Ok((stream, addr)) = listener.accept().await {
        info!("New connection from {addr}");

        // let acceptor = acceptor.clone();
        // let stream = acceptor.accept(stream)
        //     .await.expect("Failed to accept TLS connection");

        tokio::spawn(async move { handle_connection(stream, addr).await });
    }
}

async fn handle_connection(stream: TcpStream, addr: SocketAddr) {
    // Accept the WebSocket connection using the tcp stream
    let ws_stream = accept_async(stream)
        .await
        .expect("Failed to accept WebSocket connection");
    info!("WebSocket connection accepted");

    notify_rust::Notification::new()
        .summary("New device connected via JetStream")
        .body(&format!("IP: {addr}"))
        .show()
        .unwrap();

    // Split the WebSocket stream into an inbox and outbox
    let (mut outbox, mut inbox) = ws_stream.split();

    let identity_msg = pb::Identity {
        name: SERVER_NAME.to_string(),
        host: local_ip().unwrap().to_string(),
        port: PORT as u32,
    }.encode_to_vec();

    outbox.send(WSMessage::Binary(identity_msg.into()))
        .await.expect("Failed to send identity message");


    // Do this for every message received in the inbox
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
                break;
            }
        };
    }
}