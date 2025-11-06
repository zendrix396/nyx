use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, RunEvent,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

pub mod modules;
use modules::io_controller;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
        .invoke_handler(tauri::generate_handler![
            io_controller::execute_mouse_move,
            io_controller::execute_mouse_click,
            io_controller::execute_key_press,
            io_controller::execute_key_release,
            io_controller::execute_type_string,
            io_controller::test_io
        ])
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

            // Create menu items
            let toggle_i = MenuItem::with_id(app, "toggle", "Show/Hide Agent", true, None::<&str>)?;
            let settings_i = MenuItem::with_id(app, "settings", "Settings...", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            // Create menu with items
            let menu = Menu::with_items(app, &[&toggle_i, &settings_i, &quit_i])?;

            // Build the tray icon with menu and event handlers
            // Decode the PNG image to get RGBA data and dimensions
            let image_bytes = include_bytes!("../icons/image.png");
            let img = image::load_from_memory(image_bytes)
                .map_err(|e| {
                    tauri::Error::Io(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Failed to load image: {}", e),
                    ))
                })?;
            let rgba_img = img.to_rgba8();
            let (width, height) = rgba_img.dimensions();
            let rgba_data = rgba_img.into_raw();

            let tray_icon = tauri::image::Image::new_owned(rgba_data, width, height);

            let _tray = TrayIconBuilder::new()
                .icon(tray_icon)
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "toggle" => {
                            if let Some(window) = app.get_webview_window("main") {
                                toggle_window_visibility(&window);
                            }
                        }
                        "settings" => {
                            println!("Settings menu item clicked!");
                            // In the future, you would open a settings window here
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {
                            println!("menu item {:?} not handled", event.id);
                        }
                    }
                })
                .on_tray_icon_event(|tray, event| match event {
                    TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } => {
                        // Show and focus the main window on left click
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.unminimize();
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .build(app)?;

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
