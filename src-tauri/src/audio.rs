
use std::{io::BufReader, fs::File};
use rodio::{Decoder, OutputStreamHandle, Sink};

pub type AudioSource = rodio::source::Buffered<Decoder<BufReader<File>>>;

pub struct AudioPlayer {
    stream_handle: OutputStreamHandle,
    audio_source: AudioSource,

    sink: Option<Sink>
}

impl AudioPlayer {

    pub fn new(stream_handle: OutputStreamHandle, audio_source: AudioSource) -> Self {

        AudioPlayer {
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

