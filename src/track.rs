use std::cmp::Ordering;
use crate::delimiters::{END_OF_FIELD, END_OF_RECORD};


#[derive(Clone)]
pub struct Track {
    pub track_name: String,
    pub artist: String,
    pub album: String,
    pub track_number: String,  // Yes, a string, because of hidden tracks on some albums
    pub path: String,
}

impl Track {
    pub fn load(data: String) -> Track {
        let mut track_details_pos = 0;
        let mut track_name = String::new();
        let mut artist = String::new();
        let mut album = String::new();
        let mut track_number = String::new();
        let mut path = String::new();

        for c in data.chars() {
            if c == END_OF_FIELD {
                track_details_pos += 1;
                continue;
            }

            if c == END_OF_RECORD {
                return Track {
                    track_name,
                    artist,
                    album,
                    track_number,
                    path,
                };
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
        panic!("Could not create track, data had no end marker.");
    }

    pub fn dump(&self) -> String {
        format!(
            "{name}{field_end}{artist}{field_end}{album}{field_end}{track}{field_end}{path}{record_end}",
            name = &self.track_name,
            artist = &self.artist,
            album = &self.album,
            track = &self.track_number,
            path = &self.path,
            field_end = END_OF_FIELD,
            record_end = END_OF_RECORD,
        )
    }

    pub fn order_by_track(&self, other: &Self) -> Ordering {
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

    pub fn order_by_artist_and_album(&self, other: &Self) -> Ordering {
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
