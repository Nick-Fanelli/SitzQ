
use std::{io::BufReader, fs::File, sync::atomic::{AtomicU64, Ordering}};
use rodio::{Decoder, OutputStreamHandle, Sink};

pub type AudioSource = rodio::source::Buffered<Decoder<BufReader<File>>>;

pub struct AudioPlayer {
    id: u64,
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

    pub fn get_id(&self) -> u64 { self.id }

}

