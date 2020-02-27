pub struct MusicLibrary {
    name: String,
    path: String,
}

impl MusicLibrary {
    pub fn new(name: String, path: String) -> MusicLibrary {
        MusicLibrary{
            name,
            path,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}
