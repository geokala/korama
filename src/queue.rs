use crate::playlist::Playlist;
use crate::track::Track;
use std::fs::File;
use std::io::BufReader;
use rodio::Sink;
use std::thread;

use crate::shared::Saveable;


pub struct Queue {
    playlist: Option<Playlist>,
    history: Vec<Track>,
    sink: rodio::Sink,
    is_playing: bool,
}

impl Queue {
    pub fn new() -> Queue {
        let device = rodio::default_output_device().unwrap();
        Queue{
            playlist: None,
            history: Vec::new(),
            sink: rodio::Sink::new(&device),
            is_playing: false,
        }
    }

    pub fn use_playlist(&mut self, playlist: Playlist) {
        self.playlist = Some(playlist);
    }

    pub fn get_playlist(self) -> Option<Playlist> {
        self.playlist
    }

    fn play_handler(&mut self) {
        while self.is_playing {
            let next_track = self.playlist.as_mut().unwrap().next();
            match next_track {
                Some(track) => {
                    self.history.push(track.clone());
                    let file = File::open(&track.path).unwrap();
                    let source = match rodio::Decoder::new(BufReader::new(file)) {
                        Ok(src) => src,
                        // TODO: This should be logging, not panicking.
                        Err(err) => panic!("Could not play file: {}: {:#?}", &track.path, err),
                    };
                    self.sink.append(source);
                    self.sink.play();
                    self.sink.sleep_until_end();
                },
                None => self.is_playing = false,
            };
        };
    }

    pub fn play(&mut self) {
        thread::spawn(self.play_handler());
    }

    pub fn get_history(&self) -> Vec<Track> {
        self.history.clone()
    }
}
