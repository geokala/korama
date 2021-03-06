use crate::delimiters::{END_OF_FIELD, END_OF_HEADER};
use crate::music_library::MusicLibrary;
use crate::track::Track;
use crate::shared::{DynamicSource, Saveable};
use rand::Rng;
use std::cmp::min;
use std::ffi::OsStr;
use std::fs::read_to_string;
use std::path::PathBuf;

const EXTENSION: &str = "playlist";


#[derive(Clone)]
pub struct Playlist {
    name: String,
    tracks: Vec<Track>,
    window: Vec<Track>,
    dynamic_playlist_sources: Vec<Playlist>,
    dynamic_library_sources: Vec<MusicLibrary>,
    pos: Option<usize>,
}

impl Playlist {
    pub fn new(name: String) -> Playlist {
        Playlist{
            name,
            tracks: Vec::new(),
            window: Vec::new(),
            dynamic_playlist_sources: Vec::new(),
            dynamic_library_sources: Vec::new(),
            pos: None,
        }
    }

    pub fn load(saved_playlist_path: String, saved_playlist_name: String) -> Playlist {
        let mut playlist_path = PathBuf::from(&saved_playlist_path);
        playlist_path.push(OsStr::new(&format!("{}.{}", &saved_playlist_name, &EXTENSION)));

        let saved_data = match read_to_string(&playlist_path) {
            Ok(data) => data,
            Err(err) => panic!("Could not load playlist from {}: {:#?}", playlist_path.display(), err),
        };

        let header_details = Playlist::process_save_header(&saved_data);

        let tracks = Playlist::load_tracks(&saved_data);

        let pos:Option<usize>;
        if header_details[1].len() == 0 {
            pos = None;
        } else {
            pos = match header_details[1].parse::<usize>() {
                Ok(pos) => Some(pos),
                Err(err) => panic!("Could not parse playlist position in {}: {:#?}", playlist_path.display(), err),
            }
        };

        Playlist{
            name: header_details[0].to_string(),
            pos: pos,
            tracks: tracks,
            window: Vec::new(),
            dynamic_playlist_sources: Vec::new(),
            dynamic_library_sources: Vec::new(),
        }
    }

    pub fn reset_position(&mut self) {
        self.pos = None;
    }

    pub fn add_track(&mut self, track: Track) {
        &self.tracks.push(track);
    }

    pub fn remove_track(&mut self, index: usize) {
        &self.tracks.remove(index);
    }

    pub fn prev(&mut self) -> Option<Track> {
        let mut result = None;
        match self.pos {
            Some(pos) => {
                if pos == 0 {
                    self.pos = None;
                    result = None;
                } else {
                    self.pos = Some(pos - 1);
                    match self.tracks.get(self.pos.unwrap()) {
                        Some(track) => result = Some(track.clone()),
                        None => (),
                    };
                }
            }
            None => (),
        };
        result
    }

    fn add_to_window(&mut self, track: Track) {
        self.window.push(track.clone());
        if self.window.len() > self.get_window_size() {
            self.window.remove(0);
        };
    }

    fn get_window_size(&self) -> usize {
        let mut window_size = 0;
        for source in self.dynamic_playlist_sources.clone() {
            window_size += source.get_weight()
        };
        for source in self.dynamic_library_sources.clone() {
            window_size += source.get_weight()
        };

        // Effectively this is rounding up the division
        window_size += 1;
        window_size = window_size / 2;

        // Make the default window size just less than half of a smaller source set
        // Otherwise, with e.g. 3 tracks we will always play them in an order that is
        // determined randomly one.
        window_size -= 1;

        // Arbitrary max window size
        if window_size > 30 {
            window_size = 30;
        };

        window_size
    }

    fn get_random_next_track(&self) -> Option<Track> {
        let mut next_track = Vec::new();
        let mut dyn_weight = 0;
        for source in self.dynamic_playlist_sources.clone() {
            dyn_weight += source.get_weight()
        };
        for source in self.dynamic_library_sources.clone() {
            dyn_weight += source.get_weight()
        };
        if dyn_weight > 0 {
            let mut dyn_source_num = rand::thread_rng().gen_range(1, dyn_weight);
            for source in self.dynamic_playlist_sources.clone() {
                let source_weight = source.get_weight();
                if dyn_source_num <= source_weight {
                    next_track.push(source.get_random_track());
                    dyn_weight = 0;
                    break;
                } else {
                    dyn_source_num -= source_weight;
                };
            };
            if dyn_weight > 0 {
                for source in self.dynamic_library_sources.clone() {
                    let source_weight = source.get_weight();
                    if dyn_source_num <= source_weight {
                        next_track.push(source.get_random_track());
                        break;
                    } else {
                        dyn_source_num -= source_weight;
                    };
                };
            };
            match next_track.get(0) {
                Some(track) => Some(track.clone()),
                None => None,
            }
        } else {
            None
        }
    }

    pub fn get(&self, pos: usize) -> Option<Track> {
        match self.tracks.get(pos) {
            Some(track) => Some(track.clone()),
            None => None,
        }
    }

    pub fn add_dynamic_playlist_source(&mut self, source: Playlist) {
        self.dynamic_playlist_sources.push(source.clone());
    }

    pub fn add_dynamic_library_source(&mut self, source: MusicLibrary) {
        self.dynamic_library_sources.push(source.clone());
    }

    pub fn get_dynamic_playlist_sources(&self) -> Vec<Playlist> {
        self.dynamic_playlist_sources.clone()
    }

    pub fn get_dynamic_library_sources(&self) -> Vec<MusicLibrary> {
        self.dynamic_library_sources.clone()
    }
}

impl Saveable for Playlist {
    fn get_extension(&self) -> &str {
        EXTENSION
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_tracks(&self) -> Vec<Track> {
        self.tracks.clone()
    }

    fn get_header(&self) -> String {
        let mut header = String::new();

        let pos_string = match &self.pos {
            Some(pos) => pos.to_string(),
            None => String::from(""),
        };

        // Generate header
        header.push_str(&self.name);
        header.push(END_OF_FIELD);
        header.push_str(&pos_string);
        header.push(END_OF_HEADER);

        header
    }
}

impl DynamicSource for Playlist {
    fn get_tracks(&self) -> Vec<Track> {
        self.tracks.clone()
    }
}

impl Iterator for Playlist {
    type Item = Track;

    fn next(&mut self) -> Option<Track> {
        match self.pos {
            Some(pos) => self.pos = Some(min(pos + 1, self.tracks.len())),
            None => self.pos = Some(0),
        };

        if self.pos.unwrap() < self.tracks.len() {
            match self.tracks.get(self.pos.unwrap()) {
                Some(track) => Some(track.clone()),
                None => None,
            }
        } else {
            let mut next_track:Option<Track> = None;
            let window = self.window.clone();
            while next_track == None {
                next_track = self.get_random_next_track();
                match &next_track {
                    Some(track) => {
                        if window.contains(&track) {
                            next_track = None;
                        };
                    },
                    // If we receive None then for some reason we can't get a
                    // next track, so return it.
                    None => break,
                };
            };
            match next_track {
                Some(track) => {
                    self.add_to_window(track.clone());
                    self.add_track(track.clone());
                    Some(track)
                },
                None => None,
            }
        }
    }
}
