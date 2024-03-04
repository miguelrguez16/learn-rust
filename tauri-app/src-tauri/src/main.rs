// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

fn generate_menu() -> SystemTrayMenu {
    let quit = CustomMenuItem::new("quit_app", "Quit");
    let hide = CustomMenuItem::new("close_window", "Hide");
    let show = CustomMenuItem::new("open_window", "Show");
    let open_dev_tools = CustomMenuItem::new("open_dev_tools", "Open dev tools");
    return SystemTrayMenu::new()
        .add_item(hide)
        .add_item(show)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(open_dev_tools);
}

use tauri::Manager;

fn main() {
    let tray = SystemTray::new().with_menu(generate_menu());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit_app" => {
                    std::process::exit(0);
                }
                "close_window" => {
                    app.get_window("main").unwrap().minimize().unwrap();
                }
                "open_window" => {
                    app.get_window("main").unwrap().maximize().unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
