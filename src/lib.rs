use id3::Tag;
use std::ffi::OsStr;
use std::path::Path;

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
                if entry.is_file() && entry.extension().unwrap() == OsStr::new(".mp3") {
                    self.add_track_details(&entry);
                } else if entry.is_dir() {
                    scan_paths.push(entry.to_path_buf());
                }
            }
        }
    }

    fn add_track_details(&mut self, path: &Path) {
        let tags = Tag::read_from_path(path).unwrap();
        self.tracks.push(
            Track {
                track_name: tags.get("TIT2").unwrap().to_string(),
                artist: tags.get("TPE1").unwrap().to_string(),
                album: tags.get("TALB").unwrap().to_string(),
                track_number: tags.get("TRCK").unwrap().to_string(),
                path: String::from(path.to_str().unwrap()),
            }
        );
    }

    pub fn get_tracks_by_title(&self) -> &Vec<Track> {
        &self.tracks
    }
}

pub struct Track {
    pub track_name: String,
    pub artist: String,
    pub album: String,
    pub track_number: String,  // Yes, a string, because of hidden tracks on some albums
    pub path: String,
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        self.track_name == other.track_name && self.artist == other.artist && self.album == other.album && self.track_number == other.track_number && self.path == other.path
    }
}
