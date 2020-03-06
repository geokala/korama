use id3::Tag;
use std::cmp::Ordering;
use std::ffi::OsStr;
use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::{Path, PathBuf};

// Finding out what is valid in id3v2 tags turned out to be aggravating.
// It might be possible for these to be used, but they will not display well
// in most cases, so they'll probably be acceptable.
const END_OF_FIELD: char = '\u{1f}';
const END_OF_RECORD: char = '\u{1e}';
const END_OF_HEADER: char = '\u{1d}';

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
        library_path.push(OsStr::new(&format!("{}.lib", &saved_library_name)));

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

    fn process_save_header(data: &str) -> Vec<String> {
        let mut field_pos = 0;
        let mut name = String::new();
        let mut path = String::new();

        for c in data.chars() {
            if c == END_OF_FIELD {
                field_pos += 1;
            } else if c == END_OF_HEADER {
                return vec!(name, path);
            } else if field_pos == 0 {
                name.push(c);
            } else {
                path.push(c);
            }
        }
        panic!("Failed to load header!");
    }

    fn load_tracks(data: &str) -> Vec<Track> {
        let mut header_read_complete = false;

        let mut track_details_pos = 0;
        let mut track_name = String::new();
        let mut artist = String::new();
        let mut album = String::new();
        let mut track_number = String::new();
        let mut path = String::new();

        let mut tracks: Vec<Track>  = Vec::new();

        // Save file structure:
        // header:
        // <library name><END_OF_FIELD><library path><END_OF_HEADER>
        // Zero or more tracks:
        // <track name><END_OF_FIELD><artist><END_OF_FIELD><album><END_OF_FIELD><track_number><END_OF_FIELD><path><END_OF_RECORD>

        for c in data.chars() {
            if c == END_OF_HEADER {
                header_read_complete = true;
                continue;
            }

            if ! header_read_complete {
                continue;
            }

            if c == END_OF_RECORD {
                tracks.push(Track {
                    track_name: track_name.clone(),
                    artist: artist.clone(),
                    album: album.clone(),
                    track_number: track_number.clone(),
                    path: path.clone(),
                });

                track_details_pos = 0;
                track_name = String::new();
                artist = String::new();
                album = String::new();
                track_number = String::new();
                path = String::new();
                continue;
            }

            if c == END_OF_FIELD {
                track_details_pos += 1;
                continue;
            }

            match track_details_pos {
                0 => track_name.push(c),
                1 => artist.push(c),
                2 => album.push(c),
                3 => track_number.push(c),
                4 => path.push(c),
                _ => panic!("Found too many fields in track."),
            };
        }
        tracks
    }

    pub fn get_name(&self) -> &str {
        &self.name
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

    pub fn save(&self, library_storage_path: String) {
        let mut library_path = PathBuf::from(library_storage_path);
        library_path.push(OsStr::new(&format!("{}.lib", &self.name)));

        let mut data = String::new();

        // Generate header
        data.push_str(&self.name);
        data.push(END_OF_FIELD);
        data.push_str(&self.path);
        data.push(END_OF_HEADER);

        // Add tracks
        for track in &self.tracks {
            data.push_str(&track.track_name);
            data.push(END_OF_FIELD);
            data.push_str(&track.artist);
            data.push(END_OF_FIELD);
            data.push_str(&track.album);
            data.push(END_OF_FIELD);
            data.push_str(&track.track_number);
            data.push(END_OF_FIELD);
            data.push_str(&track.path);
            data.push(END_OF_RECORD);
        };

        let mut library_file = match File::create(&library_path) {
            Ok(file) => file,
            Err(err) => panic!("Could not create {}: {:#?}", library_path.display(), err),
        };
        match library_file.write_all(data.as_bytes()) {
            Ok(_) => {},
            Err(err) => panic!("Could not write to {}: {:#?}", library_path.display(), err),
        };
        match library_file.sync_all() {
            Ok(_) => {},
            Err(err) => panic!("Could not complete write to {}: {:#?}", library_path.display(), err),
        };
    }
}

#[derive(Clone)]
pub struct Track {
    pub track_name: String,
    pub artist: String,
    pub album: String,
    pub track_number: String,  // Yes, a string, because of hidden tracks on some albums
    pub path: String,
}

impl Track {
    fn order_by_track(&self, other: &Self) -> Ordering {
        if self.track_name > other.track_name {
            Ordering::Greater
        } else if self.track_name < other.track_name {
            Ordering::Less
        } else {
            // Artist name breaks ties
            if self.artist > other.artist {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    }

    fn order_by_artist_and_album(&self, other: &Self) -> Ordering {
        if self.artist > other.artist {
            Ordering::Greater
        } else if self.artist < other.artist {
            Ordering::Less
        } else {
            // Same artist
            if self.album > other.album {
                Ordering::Greater
            } else if self.album < other.album {
                Ordering::Less
            } else {
                // Same album
                if self.track_number > other.track_number {
                    Ordering::Greater
                } else if self.track_number < other.track_number {
                    Ordering::Less
                } else {
                    // Somebody forgot to put track numbers
                    // We'll break ties on track name here
                    if self.track_name > other.track_name {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                }
            }
        }
    }
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        self.track_name == other.track_name && self.artist == other.artist && self.album == other.album && self.track_number == other.track_number && self.path == other.path
    }
}
