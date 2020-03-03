use id3::Tag;
use std::cmp::Ordering;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

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
        MusicLibrary{
            name: String::from("Hello"),
            path: String:: from("Yes"),
            tracks: Vec::new(),
        }
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
        // Finding out what is valid in id3v2 tags turned out to be aggravating.
        // It might be possible for these to be used, but they will not display well
        // in most cases, so they'll probably be acceptable.
        let end_of_field = std::char::from_digit(31, 10).unwrap();
        let end_of_record = std::char::from_digit(30, 10).unwrap();
        let end_of_header = std::char::from_digit(29, 10).unwrap();

        let mut library_path = PathBuf::from(library_storage_path);
        library_path.push(OsStr::new(&self.name));
        library_path.push(OsStr::new(".lib"));

        let mut data = String::new();

        // Generate header
        data.push_str(&self.name);
        data.push(end_of_field);
        data.push_str(&self.path);
        data.push(end_of_header);

        // Add tracks
        for track in &self.tracks {
            data.push_str(&track.track_name);
            data.push(end_of_field);
            data.push_str(&track.artist);
            data.push(end_of_field);
            data.push_str(&track.album);
            data.push(end_of_field);
            data.push_str(&track.track_number);
            data.push(end_of_field);
            data.push_str(&track.path);
            data.push(end_of_record);
        };

        let mut library_file = match File::create(&self.path) {
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
        let result;
        if self.track_name > other.track_name {
            result = Ordering::Greater;
        } else if self.track_name < other.track_name {
            result = Ordering::Less;
        } else {
            // Artist name breaks ties
            if self.artist > other.artist {
                result = Ordering::Greater;
            } else {
                result = Ordering::Less;
            }
        }
        result
    }

    fn order_by_artist_and_album(&self, other: &Self) -> Ordering {
        let result;
        if self.artist > other.artist {
            result = Ordering::Greater;
        } else if self.artist < other.artist {
            result = Ordering::Less;
        } else {
            // Same artist
            if self.album > other.album {
                result = Ordering::Greater;
            } else if self.album < other.album {
                result = Ordering::Less;
            } else {
                // Same album
                if self.track_number > other.track_number {
                    result = Ordering::Greater;
                } else if self.track_number < other.track_number {
                    result = Ordering::Less;
                } else {
                    // Somebody forgot to put track numbers
                    // We'll break ties on track name here
                    if self.track_name > other.track_name {
                        result = Ordering::Greater;
                    } else {
                        result = Ordering::Less;
                    }
                }
            }
        }
        result
    }
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        self.track_name == other.track_name && self.artist == other.artist && self.album == other.album && self.track_number == other.track_number && self.path == other.path
    }
}
