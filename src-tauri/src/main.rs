// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod connections;

fn main() {
    // Build and Run Tauri
    tauri::Builder::default()

        .invoke_handler(tauri::generate_handler![
            connections::start_transmission_server,
            connections::start_reception_server
        ])

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
}