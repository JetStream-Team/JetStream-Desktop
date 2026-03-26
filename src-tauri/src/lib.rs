use tauri::{
    AppHandle, Manager, menu::{Menu, MenuItem}, tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent}
};

pub mod certs;
pub mod protobuf_message;
pub mod message_handlers;
pub mod websocket;

pub const SERVER_NAME: &str = "Mathew's JetStream";
pub const PORT: u16 = 8000;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();
            let args: Vec<String> = std::env::args().collect();
            let window = app_handle.get_webview_window("main")
                    .expect("Failed to get webview window");
            let window_clone = window.clone();

            // Start minimized if argument is passed
            if args.contains(&"--start-minimized".to_string()) {
                window.hide()
                    .expect("Failed to hide webview window");
            }

            // Minimize to tray on window close
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    window_clone.hide()
                        .expect("Failed to hide webview window");
                }
            });

            build_tray(&app_handle)?;

            // Start the server in a seperate thread
            tokio::spawn(async move {
                websocket::start_server(app_handle).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn build_tray(app: &tauri::AppHandle) -> Result<(), tauri::Error> {
    let show_hide = MenuItem::with_id(
        app,
        "show_hide",
        "Show/Hide App",
        true,
        None::<&str>
    )?;
    let send_clipboard = MenuItem::with_id(
        app,
        "send_clipboard",
        "Send Clipboard",
        true,
        None::<&str>
    )?;
    let quit = MenuItem::with_id(
        app,
        "quit",
        "Quit",
        true,
        None::<&str>
    )?;

    let menu = Menu::with_items(
        app,
        &[&show_hide, &send_clipboard, &quit]
    )?;

    TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip("Disconnected")
        .title("Disconnected")
        .menu(&menu)
        .show_menu_on_left_click(false) // We handle left-click manually
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show_hide" => {
                toggle_window_visibility(app);
            },
            "send_clipboard" => {
                message_handlers::clipboard::send_clipboard();
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                toggle_window_visibility(app);
            }
        })
        .build(app)?;


    Ok(())
}

fn toggle_window_visibility(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}