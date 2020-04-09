use crate::playlist::Playlist;


pub struct Queue {
    playlist: Option<Playlist>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue{
            playlist: None,
        }
    }

    pub fn use_playlist(&mut self, playlist: Playlist) {
        self.playlist = Some(playlist);
    }

    pub fn get_playlist(self) -> Option<Playlist> {
        self.playlist
    }
}
