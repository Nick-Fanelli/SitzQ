// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;

use std::collections::HashMap;
use std::sync::Mutex;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;
use std::thread;
use std::time::Duration;

use rodio::OutputStreamHandle;
use rodio::{OutputStream, Source};

use tauri::State;

use audio::AudioSource;
use audio::AudioPlayer;

type MultithreadedAppState = Arc<Mutex<AppState>>;

#[tauri::command]
fn play_sound(audio_path: String, state: State<MultithreadedAppState>) -> u64 {

    let state = state.lock().unwrap();

    println!("Playing Sound File: {}", audio_path);

    let mut sources = state.sources.lock().unwrap();
    let mut audio_players = state.audio_players.lock().unwrap();

    if !sources.contains_key(&audio_path) {

        let file = File::open(&audio_path).expect("Error opening file");
        let source = rodio::Decoder::new(BufReader::new(file)).expect("Error loading source").buffered();

        sources.insert(audio_path.clone(), source);

    }

    let source = sources.get(&audio_path).expect("Error getting source");

    // TODO: Fix the amount of clones happening
    let mut audio_player = AudioPlayer::new(
        &state.audio_players_atomic, 
        state.stream_handle.clone(),
        source.clone());

    let audio_player_id = audio_player.get_id();

    audio_player.play();
    audio_players.push(audio_player);

    return audio_player_id;

}

#[tauri::command]
fn print_stats(state: State<MultithreadedAppState>) {

    let state = state.lock().unwrap();

    let audio_players = state.audio_players.lock().unwrap();

    println!("Num of Audio Players: {}", &audio_players.len());

}

struct AppState {
    stream_handle: OutputStreamHandle,

    sources: Mutex<HashMap<String, AudioSource>>,

    audio_players_atomic: AtomicU64,
    audio_players: Mutex<Vec<AudioPlayer>>
}

impl AppState {
    fn new(stream_handle: OutputStreamHandle) -> Self {
        AppState { 
            stream_handle,

            sources: Default::default(),

            audio_players_atomic: AtomicU64::new(0),
            audio_players: Default::default()
        }
    }

    fn cleanup_audio_players(&mut self) {

        let mut audio_players = self.audio_players.lock().unwrap();
        audio_players.retain(|player| !player.is_sink_done());

    }
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
            play_sound,
            print_stats
        ])

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
}
