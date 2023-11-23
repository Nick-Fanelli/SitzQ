// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::BufReader;

use rodio::{Sink, OutputStream};

use tauri::State;

#[tauri::command]
fn play_sound(audio_path: String, state: State<AppState>) {

    println!("Playing Sound File: {}", audio_path);

    let file = File::open(audio_path).expect("Error opening filepath");
    let source = rodio::Decoder::new(BufReader::new(file)).expect("Error loading source");

    state.sink.append(source);

}

struct AppState {
    sink: Sink,
}

impl AppState {
    fn new(sink: Sink) -> Self {
        AppState { sink }
    }
}

fn main() {

    // let device = cpal::default_host().default_output_device().expect("Error getting default output device");

    // Create an output stream
    let (_stream, stream_handle) = OutputStream::try_default().expect("Error creating output stream");

    // Play the sound on the default device
    let sink = Sink::try_new(&stream_handle).expect("Error creating sink");

    let app_state = AppState::new(sink);

    tauri::Builder::default()
        .manage(app_state)

        .invoke_handler(tauri::generate_handler![play_sound])

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
