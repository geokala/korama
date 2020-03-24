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
    dynamic_playlist_sources: Vec<Playlist>,
    dynamic_library_sources: Vec<MusicLibrary>,
    pos: Option<usize>,
}

impl Playlist {
    pub fn new(name: String) -> Playlist {
        Playlist{
            name,
            tracks: Vec::new(),
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

    pub fn next(&mut self) -> Option<&Track> {
        match self.pos {
            Some(pos) => self.pos = Some(min(pos + 1, self.tracks.len())),
            None => self.pos = Some(0),
        };

        if self.pos.unwrap() >= self.tracks.len() {
            self.tracks.get(self.pos.unwrap())
        } else {
            match self.get_random_next_track() {
                Some(track) => Some(&track),
                None => None,
            }
        }
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

    fn get_random_next_track(&mut self) -> Option<Track> {
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
                    self.add_track(source.get_random_track());
                    break;
                } else {
                    dyn_source_num -= source_weight;
                };
            };
            match self.tracks.get(self.tracks.len() - 1) {
                Some(track) => Some(*track),
                None => None,
            }
        } else {
            None
        }
    }

    pub fn get(&self, pos: usize) -> Option<&Track> {
        self.tracks.get(pos)
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
