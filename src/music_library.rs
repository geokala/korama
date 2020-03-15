use id3::Tag;
use std::ffi::OsStr;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use crate::delimiters::{END_OF_FIELD, END_OF_HEADER};
use crate::shared::Saveable;
use crate::track::Track;

const EXTENSION: &str = "lib";


pub struct MusicLibrary {
    name: String,
    path: String,
    tracks: Vec<Track>,
}

impl MusicLibrary {
    pub fn new(name: String, path: String) -> MusicLibrary {
        MusicLibrary{
            name,
            path,
            tracks: Vec::new(),
        }
    }

    pub fn load(saved_library_path: String, saved_library_name: String) -> MusicLibrary {
        let mut library_path = PathBuf::from(&saved_library_path);
        library_path.push(OsStr::new(&format!("{}.{}", &saved_library_name, &EXTENSION)));

        let saved_data = match read_to_string(&library_path) {
            Ok(data) => data,
            Err(err) => panic!("Could not load library from {}: {:#?}", library_path.display(), err),
        };

        let header_details = MusicLibrary::process_save_header(&saved_data);

        let tracks = MusicLibrary::load_tracks(&saved_data);

        MusicLibrary{
            name: header_details[0].to_string(),
            path: header_details[1].to_string(),
            tracks: tracks,
        }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn scan(&mut self) {
        let mut scan_paths = Vec::new();
        scan_paths.push(Path::new(&self.path).to_path_buf());
        while scan_paths.len() > 0 {
            let current_path = scan_paths.pop().unwrap();
            for entry in current_path.read_dir().expect("Could not read {}.") {
                let entry = entry.unwrap().path();
                if entry.is_file() {
                    let extension = match entry.extension() {
                        Some(osstr) => osstr,
                        None => OsStr::new(""),
                    };
                    if extension == OsStr::new("mp3") { 
                        self.add_track_details(&entry);
                    }
                } else if entry.is_dir() {
                    scan_paths.push(entry.to_path_buf());
                }
            }
        }
    }

    fn add_track_details(&mut self, path: &Path) {
        let tags = match Tag::read_from_path(path) {
            Ok(res) => res,
            Err(_) => {
                println!("Unable to read id3v2 tags for {}", path.display());
                return;
            },
        };

        let track_name: String = match tags.get("TIT2") {
            Some(res) => res.to_string(),
            None => {
                println!("Could not get track name (id3v2 TIT2 tag) for {}", path.display());
                return;
            },
        };
        let artist: String = match tags.get("TPE1") {
            Some(res) => res.to_string(),
            None => {
                println!("Could not get artist (id3v2 TPE1 tag) for {}", path.display());
                return;
            },
        };
        let album: String = match tags.get("TALB") {
            Some(res) => res.to_string(),
            None => String::from(""),  // Album is not required
        };
        let track_number: String = match tags.get("TRCK") {
            Some(res) => res.to_string(),
            None => String::from(""),  // Track number is not required
        };

        self.tracks.push(
            Track {
                track_name,
                artist,
                album,
                track_number,
                path: String::from(path.to_str().unwrap()),
            }
        );
    }

    pub fn get_tracks_by_title(&self) -> Vec<Track> {
        let tracks = &mut self.tracks.clone();
        tracks.sort_by(|a, b| a.order_by_track(b));
        tracks.clone()
    }

    pub fn get_tracks_by_artist_and_album(&self) -> Vec<Track> {
        let tracks = &mut self.tracks.clone();
        tracks.sort_by(|a, b| a.order_by_artist_and_album(b));
        tracks.clone()
    }
}

impl Saveable for MusicLibrary {
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

        // Generate header
        header.push_str(&self.name);
        header.push(END_OF_FIELD);
        header.push_str(&self.path);
        header.push(END_OF_HEADER);

        header
    }
}
