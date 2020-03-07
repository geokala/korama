pub struct Playlist {
    name: String,
}

impl Playlist {
    pub fn new(name: String) -> Playlist {
        Playlist{
            name,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
