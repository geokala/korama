use crate::track::Track;

pub struct Playlist {
    name: String,
    tracks: Vec<Track>,
}

impl Playlist {
    pub fn new(name: String) -> Playlist {
        Playlist{
            name,
            tracks: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_tracks(self) -> Vec<Track> {
        self.tracks
    }

    pub fn add_track(&mut self, track: Track) {
        &self.tracks.push(track);
    }
}
