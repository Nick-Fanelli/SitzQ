use std::{io::BufReader, fs::File, sync::atomic::{AtomicU64, Ordering}};

use tauri::State;

use rodio::{Decoder, OutputStreamHandle, Sink, Source};

use crate::app_state::MultithreadedAppState;

pub type AudioSource = rodio::source::Buffered<Decoder<BufReader<File>>>;
pub type AudioPlayerID = u64;

pub struct AudioPlayer {
    pub id: AudioPlayerID,
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

        match &self.sink {

            Some(sink) => {
                
                if sink.is_paused() {
                    sink.play();
                } else {
                    sink.append(self.audio_source.clone());
                }

            },

            None => {
                self.sink = Some(Sink::try_new(&self.stream_handle).expect("Error creating sink"));

                match &self.sink {

                    Some(sink) => {
                        sink.append(self.audio_source.clone());
                    },

                    None => {
                        eprintln!("ERROR: Problem creating sink");
                    }

                }
                
            }

        }

    }

    pub fn pause(&self) {

        match &self.sink {

            Some(sink) => {
                sink.pause();
            },

            None => {
                eprintln!("WARNING: Calling pause on audio player without an optional sink!");
            }

        }

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

fn get_audio_player(audio_players: &Vec<AudioPlayer>, audio_player_id: AudioPlayerID) -> Option<&AudioPlayer> {

    for audio_player in audio_players {

        if audio_player.id == audio_player_id {
            return Some(audio_player);
        }

    }

    return None;

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

#[tauri::command]
pub fn pause_audio_player(audio_player_id: AudioPlayerID, state: State<MultithreadedAppState>) -> Result<(), String> {

    println!("Pausing Audio Player ID: {}", audio_player_id);

    let state = state.lock().unwrap();
    let audio_players = state.audio_players.lock().unwrap();

    let audio_player = get_audio_player(&audio_players, audio_player_id);

    match audio_player {

        Some(audio_player) => {

            audio_player.pause();

            return Ok(());

        },

        None => {

            return Err("Audio player has been garbage collected or never existed".into());

        }

    }

}