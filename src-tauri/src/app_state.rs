use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;

use rodio::OutputStreamHandle;

use crate::audio::AudioSource;
use crate::audio::AudioPlayer;

pub type MultithreadedAppState = Arc<Mutex<AppState>>;

pub struct AppState {
    pub stream_handle: OutputStreamHandle,

    pub sources: Mutex<HashMap<String, AudioSource>>,

    pub audio_players_atomic: AtomicU64,
    pub audio_players: Mutex<Vec<AudioPlayer>>
}

impl AppState {
    pub fn new(stream_handle: OutputStreamHandle) -> Self {
        AppState { 
            stream_handle,

            sources: Default::default(),

            audio_players_atomic: AtomicU64::new(0),
            audio_players: Default::default()
        }
    }

    pub fn cleanup_audio_players(&mut self) {

        let mut audio_players = self.audio_players.lock().unwrap();
        audio_players.retain(|player| !player.is_sink_done());

    }
}
