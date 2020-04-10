use crate::playlist::Playlist;
use crate::track::Track;
use std::fs::File;
use std::io::BufReader;
use rodio::Sink;


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

    pub fn play(mut self) {
        for track in self.playlist.iter() {
            let file = File::open(track.path).unwrap();
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
            self.sink.append(source);
        };
        self.sink.play()
    }

    pub fn is_playing(self) -> bool {
        // We are playing unless we are empty or paused
        !(self.sink.empty() | self.sink.is_paused())
    }

    pub fn history(self) -> Vec<Track> {
        self.history.clone()
    }
}
