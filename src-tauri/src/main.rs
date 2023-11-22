// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn test_command() {
    println!("Hello World");
}

fn main() {
    tauri::Builder::default()

        .invoke_handler(tauri::generate_handler![test_command])

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
