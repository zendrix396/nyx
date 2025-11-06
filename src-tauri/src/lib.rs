use tauri::{
    AppHandle, CustomMenuItem, Manager, RunEvent, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 1. Define the menu items
    let toggle_visibility = CustomMenuItem::new("toggle".to_string(), "Show/Hide Agent");
    let settings = CustomMenuItem::new("settings".to_string(), "Settings...");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    // 2. Create the system tray menu
    let tray_menu = SystemTrayMenu::new()
        .add_item(toggle_visibility)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(settings)
        .add_item(quit);

    // 3. Create the system tray
    let system_tray = SystemTray::new()
        .with_menu(tray_menu)
        .with_icon(tauri::Icon::Raw(
            include_bytes!("../icons/tray-icon.png").to_vec(),
        ));

    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(move |app, shortcut, event| {
                    if let (ShortcutState::Pressed, Some(window)) =
                        (event.state(), app.get_webview_window("main"))
                    {
                        if shortcut.matches(Modifiers::empty(), Code::F4) {
                            toggle_window_visibility(&window);
                        }
                    }
                })
                .build(),
        )
        // 4. Add the system_tray and its event handler to the builder
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "toggle" => {
                    if let Some(window) = app.get_webview_window("main") {
                        toggle_window_visibility(&window);
                    }
                }
                "settings" => {
                    println!("Settings menu item clicked!");
                    // In the future, you would open a settings window here
                    // let settings_window = app.get_webview_window("settings").unwrap();
                    // settings_window.show().unwrap();
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            app.global_shortcut()
                .register(Shortcut::new(None, Code::F4))?;
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| {
            if let RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        });
}

// Helper function to avoid duplicating the show/hide logic
fn toggle_window_visibility(window: &tauri::WebviewWindow) {
    if let Ok(true) = window.is_visible() {
        let _ = window.hide();
    } else {
        let _ = window.show();
        let _ = window.set_focus();
    }
}
