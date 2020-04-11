use crate::playlist::Playlist;
use crate::track::Track;
use std::fs::File;
use std::io::BufReader;
use rodio::Sink;

use crate::shared::Saveable;


pub struct Queue {
    playlist: Option<Playlist>,
    history: Vec<Track>,
    sink: rodio::Sink,
}

impl Queue {
    pub fn new() -> Queue {
        let device = rodio::default_output_device().unwrap();
        Queue{
            playlist: None,
            history: Vec::new(),
            sink: rodio::Sink::new(&device),
        }
    }

    pub fn use_playlist(&mut self, playlist: Playlist) {
        self.playlist = Some(playlist);
    }

    pub fn get_playlist(self) -> Option<Playlist> {
        self.playlist
    }

    pub fn play(&self) {
        for track in self.playlist.unwrap() {
            let file = File::open(track.path).unwrap();
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
            self.sink.append(source);
        };
        self.sink.play()
    }

    pub fn get_history(&self) -> Vec<Track> {
        self.history.clone()
    }
}
