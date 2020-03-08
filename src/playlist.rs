use crate::track::Track;
use std::cmp::min;

pub struct Playlist {
    name: String,
    tracks: Vec<Track>,
    pos: Option<usize>,
}

impl Playlist {
    pub fn new(name: String) -> Playlist {
        Playlist{
            name,
            tracks: Vec::new(),
            pos: None,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_tracks(&self) -> Vec<Track> {
        self.tracks.clone()
    }

    pub fn add_track(&mut self, track: Track) {
        &self.tracks.push(track);
    }

    pub fn remove_track(&mut self, index: usize) {
        &self.tracks.remove(index);
    }

    pub fn next(&mut self) -> Option<&Track> {
        match self.pos {
            Some(pos) => self.pos = Some(min(pos + 1, self.tracks.len())),
            None => self.pos = Some(0),
        };

        self.tracks.get(self.pos.unwrap())
    }

    pub fn prev(&mut self) -> Option<&Track> {
        let mut result = None;
        match self.pos {
            Some(pos) => {
                if pos == 0 {
                    self.pos = None;
                    result = None;
                } else {
                    self.pos = Some(pos - 1);
                    result = self.tracks.get(self.pos.unwrap());
                }
            }
            None => (),
        };
        result
    }
}
