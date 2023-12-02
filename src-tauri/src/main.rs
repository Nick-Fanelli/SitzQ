// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use app_state::AppState;

mod app_state;
mod connections;

fn main() {
    let app_state = Arc::new(Mutex::new(AppState::new()));

    tauri::Builder::default()

        .manage(app_state)

        .invoke_handler(tauri::generate_handler![
            app_state::generate_network_remote,

            connections::start_transmission_server,
            connections::start_reception_server
        ])

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
}