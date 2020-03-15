use crate::delimiters::{END_OF_FIELD, END_OF_HEADER, END_OF_RECORD};
use crate::track::Track;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub trait Saveable {
    fn save(&self, data_storage_path: String) {
        let mut data_path = PathBuf::from(data_storage_path);
        data_path.push(OsStr::new(&format!("{}.{}", &self.get_name(), &self.get_extension())));

        let mut data = String::new();

        data.push_str(&self.get_header());

        // Add tracks
        for track in &self.get_tracks() {
            data.push_str(&track.dump());
        };

        let mut data_file = match File::create(&data_path) {
            Ok(file) => file,
            Err(err) => panic!("Could not create {}: {:#?}", data_path.display(), err),
        };
        match data_file.write_all(data.as_bytes()) {
            Ok(_) => {},
            Err(err) => panic!("Could not write to {}: {:#?}", data_path.display(), err),
        };
        match data_file.sync_all() {
            Ok(_) => {},
            Err(err) => panic!("Could not complete write to {}: {:#?}", data_path.display(), err),
        };
    }

    fn process_save_header(data: &str) -> Vec<String> {
        let mut field_pos = 0;
        let mut name = String::new();
        let mut field2 = String::new();

        for c in data.chars() {
            if c == END_OF_FIELD {
                field_pos += 1;
            } else if c == END_OF_HEADER {
                return vec!(name, field2);
            } else if field_pos == 0 {
                name.push(c);
            } else {
                field2.push(c);
            }
        }
        panic!("Failed to load header!");
    }

    fn load_tracks(data: &str) -> Vec<Track> {
        let mut header_read_complete = false;

        let mut track_data = String::new();

        let mut tracks: Vec<Track> = Vec::new();

        // Save file structure:
        // header:
        // <library name><END_OF_FIELD><library path><END_OF_HEADER>
        // <zero or more tracks (see track for serialisation format)>

        for c in data.chars() {
            if c == END_OF_HEADER {
                header_read_complete = true;
                continue;
            }

            if ! header_read_complete {
                continue;
            }

            track_data.push(c);

            if c == END_OF_RECORD {
                tracks.push(Track::load(track_data));
                track_data = String::new()
            }
        }
        tracks
    }

    fn get_header(&self) -> String;
    fn get_name(&self) -> &str;
    fn get_extension(&self) -> &str;
    fn get_tracks(&self) -> Vec<Track>;
}
