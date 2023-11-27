use std::{io::BufReader, fs::File, sync::atomic::{AtomicU64, Ordering}};

use tauri::State;

use rodio::{Decoder, OutputStreamHandle, Sink, Source};

use crate::app_state::MultithreadedAppState;

pub type AudioSource = rodio::source::Buffered<Decoder<BufReader<File>>>;

pub struct AudioPlayer {
    pub id: u64,
    stream_handle: OutputStreamHandle,
    audio_source: AudioSource,

    sink: Option<Sink>
}

impl AudioPlayer {

    pub fn new(id_atomic: &AtomicU64, stream_handle: OutputStreamHandle, audio_source: AudioSource) -> Self {

        AudioPlayer {
            id: id_atomic.fetch_add(1, Ordering::SeqCst),
            stream_handle,
            audio_source,
            sink: None
        }

    }

    pub fn play(&mut self) {

        let sink = self.sink.get_or_insert_with(|| Sink::try_new(&self.stream_handle).expect("Error creating sink"));
        sink.append(self.audio_source.clone());

    }

    pub fn is_sink_done(&self) -> bool {

        match &self.sink {

            Some(sink) => {
                sink.empty()
            },

            None => { // Consider returning a result that is not true
                true
            }

        }

    }

}

#[tauri::command]
pub fn play_sound(audio_path: String, state: State<MultithreadedAppState>) -> u64 {

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

    let audio_player_id = audio_player.id;

    audio_player.play();
    audio_players.push(audio_player);

    return audio_player_id;

}