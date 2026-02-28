use log::{debug, error, info, warn};
// use crate::certs;
// use rustls::ServerConfig;
// use tokio_rustls::{TlsAcceptor, server::TlsStream};
use tokio::{net::{TcpListener, TcpStream}};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message as WSMessage;
use futures_util::{SinkExt, StreamExt};
use prost::Message;
use base64::{Engine, prelude::BASE64_STANDARD};
use notify_rust;

#[allow(dead_code)]
pub type WSInbox = futures_util::stream::SplitStream<tokio_tungstenite::WebSocketStream<TcpStream>>;
#[allow(dead_code)]
pub type WSOutbox = futures_util::stream::SplitSink<tokio_tungstenite::WebSocketStream<TcpStream>, tokio_tungstenite::tungstenite::Message>;

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
        .await.expect("Failed to bind to port {port}");
    info!("Listening on port 0.0.0.0:{port}");

    while let Ok((stream, addr)) = listener.accept().await {
        info!("New connection from {addr}");

        // let acceptor = acceptor.clone();
        // let stream = acceptor.accept(stream)
        //     .await.expect("Failed to accept TLS connection");
        
        tokio::spawn(async move {handle_connection(stream).await});
        
    }
}

async fn handle_connection(stream: TcpStream) {
    // Accept the WebSocket connection using the tcp stream
    let ws_stream = accept_async(stream)
        .await.expect("Failed to accept WebSocket connection");

    // Split the WebSocket stream into an inbox and outbox
    let (mut outbox, mut inbox) = ws_stream.split();

    // Do this for every message received in the inbox
    while let Some(result) = inbox.next().await {
        match result {
            Ok(ws_msg) => {
                match ws_msg {
                    WSMessage::Text(msg) => {
                        debug!("Received message: {msg}");
                        outbox.send(format!("Echo: {msg}").into()).await.unwrap();
                        let msg_bytes = BASE64_STANDARD.decode(msg).expect("Failed to decode base64 message");
                        handle_protobuf_message(&msg_bytes).await;
                    }
                    WSMessage::Binary(msg) => {
                        debug!("Received binary message: {msg:?}");
                        handle_protobuf_message(&msg).await;
                    }
                    WSMessage::Close(_) => {
                        info!("Client disconnected");
                        break;
                    }
                    _ => {}
                }
            },
            Err(e) => {
                error!("Error receiving message: {e}");
                break;
            }
        };
    }
}

async fn handle_protobuf_message(data: &[u8]) {
    match pb::MessageWrapper::decode(data) {
        Ok(wrapper) => {
            debug!("Decoded wrapped protobuf message: {wrapper:?}");
            match wrapper.message {
                Some(pb::message_wrapper::Message::Notification(notif)) => {
                    debug!("--- Notification Received ---");
                    debug!("Title: {}", notif.title);
                    debug!("Body:  {}", notif.body);
                    notify_rust::Notification::new()
                        .summary(&notif.title)
                        .body(&notif.body)
                        .show().unwrap();
                }
                Some(pb::message_wrapper::Message::Clipboard(cb)) => {
                    debug!("--- Clipboard Sync ---");
                    debug!("Content: {}", cb.content);
                    #[cfg(target_os = "windows")]
                    clipboard_win::set_clipboard_string(&cb.content).expect("Failed to set clipboard content");
                    
                    #[cfg(target_os = "linux")]
                    info!("Clipboard sync not implemented for Linux yet. Received content: {}", cb.content);
                    
                }
                None => {
                    warn!("Received an empty MessageWrapper");
                }
            }
        },
        Err(e) => {
            error!("Failed to decode protobuf message: {e}");
        }
    }
}