#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let show_item = MenuItemBuilder::new("Показать")
                .id("show")
                .build(app)?;

            let hide_item = MenuItemBuilder::new("Скрыть")
                .id("hide")
                .build(app)?;

            let quit_item = MenuItemBuilder::new("Выход")
                .id("quit")
                .build(app)?;

            let menu = MenuBuilder::new(app)
                .item(&show_item)
                .item(&hide_item)
                .separator()
                .item(&quit_item)
                .build()?;

            let tray_icon = Image::from_bytes(include_bytes!("../icons/icon.png"))?;

            let _tray = TrayIconBuilder::new()
                .icon(tray_icon)
                .menu(&menu)
                .tooltip("Timeline")
                .on_menu_event(|app, event| {
                    let window = app.get_webview_window("main").unwrap();

                    match event.id().as_ref() {
                        "show" => {
                            let _ = window.unminimize();
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                        "hide" => {
                            let _ = window.hide();
                        }
                        "quit" => {
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        let window = app.get_webview_window("main").unwrap();
                        let _ = window.unminimize();
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                })
                .build(app)?;

            let window = app.get_webview_window("main").unwrap();
            let win_clone = window.clone();

            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = win_clone.hide();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}