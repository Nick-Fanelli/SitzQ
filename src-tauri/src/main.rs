// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod app_state;

use std::sync::Mutex;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use rodio::OutputStream;

use tauri::State;

use app_state::AppState;
use app_state::MultithreadedAppState;

#[tauri::command]
fn print_stats(state: State<MultithreadedAppState>) {

    let state = state.lock().unwrap();

    let audio_players = state.audio_players.lock().unwrap();

    println!("Num of Audio Players: {}", &audio_players.len());

}

fn spawn_cleanup_thread(app_state: &MultithreadedAppState) {

    let cleanup_thread_app_state = Arc::clone(app_state);

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5));

            let mut state = cleanup_thread_app_state.lock().unwrap();
            state.cleanup_audio_players();
        }
    });

}

fn main() {

    // Create an output stream
    let (_stream, stream_handle) = OutputStream::try_default().expect("Error creating output stream");

    // Play the sound on the default device
    let app_state = Arc::new(Mutex::new(AppState::new(stream_handle)));

    // Garbage Collector
    spawn_cleanup_thread(&app_state);

    // Build and Run Tauri
    tauri::Builder::default()
        .manage(app_state)

        .invoke_handler(tauri::generate_handler![
            audio::play_sound,
            print_stats
        ])

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
}