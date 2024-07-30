// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod db;
mod wayland;

use tauri::SystemTray;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
use tracing::level_filters::LevelFilter;

fn main() {
    // construct a subscriber that prints formatted traces to stdout
    let subscriber = tracing_subscriber::fmt().without_time().with_max_level(LevelFilter::TRACE).finish();
    // use that subscriber to process traces emitted after this point
    tracing::subscriber::set_global_default(subscriber).expect("failed to set up tracing");

    let open_button = CustomMenuItem::new("open".to_string(), "Open Hourlasso");
    let menu = SystemTrayMenu::new().add_item(open_button);
    let tray = SystemTray::new().with_menu(menu);
    tauri::Builder::setup(tauri::Builder::default(), |app| {
        tauri::async_runtime::spawn(wayland::connect());
        Ok(())
    })
    .system_tray(tray)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
