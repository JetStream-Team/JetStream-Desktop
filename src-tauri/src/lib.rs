use serde::Serialize;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_connection
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Serialize)]
struct Connection {
    device_name: String,
    ip_address: String,
}

#[tauri::command]
async fn get_connection() -> Connection {
    let connection = Connection {
        device_name: "Mathew's Test Device".to_string(),
        ip_address: "127.0.0.1".into()
    };

    connection
}