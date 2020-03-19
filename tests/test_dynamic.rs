use std::path::PathBuf;
use korama;


#[test]
fn add_dynamic_source_playlist() {
    let mut dyn_playlist = korama::Playlist::new(String::from("Test dynamic playlist"));

    let mut source_playlist = get_playlist_source();

    dyn_playlist.add_dynamic_source(source_playlist);

    let dynamic_sources = dyn_playlist.get_dynamic_sources();

    assert_eq!(dynamic_sources.len(), 1);
    assert_eq!(dynamic_sources[0], source_playlist);

    let track = dyn_playlist.next().unwrap();

    assert!(get_playlist_titles().contains(&track.track_name));
}

#[test]
fn add_dynamic_source_library() {
    let mut dyn_playlist = korama::Playlist::new(String::from("Test dynamic playlist"));

    let mut source_library = get_library_source();

    dyn_playlist.add_dynamic_source(source_library);

    let dynamic_sources = dyn_playlist.get_dynamic_sources();

    assert_eq!(dynamic_sources.len(), 1);
    assert_eq!(dynamic_sources[0], source_library);

    let track = dyn_playlist.next().unwrap();

    assert!(get_library_titles().contains(&track.track_name));
}

#[test]
fn add_dynamic_source_library_and_playlist() {
    let mut dyn_playlist = korama::Playlist::new(String::from("Test dynamic playlist"));

    let mut source_playlist = get_playlist_source();
    let mut source_library = get_library_source();

    dyn_playlist.add_dynamic_source(source_library);
    dyn_playlist.add_dynamic_source(source_playlist);

    let dynamic_sources = dyn_playlist.get_dynamic_sources();

    assert_eq!(dynamic_sources.len(), 1);
    assert_eq!(dynamic_sources[0], source_library);

    let track = dyn_playlist.next().unwrap();

    let mut possible_names = get_playlist_titles();
    possible_names.append(&mut get_library_titles());

    assert!(possible_names.contains(&track.track_name));
}

fn get_playlist_titles() -> Vec<String> {
    vec!(
        String::from("First track"),
        String::from("Second track"),
        String::from("Third track"),
    )
}

fn get_library_titles() -> Vec<String> {
    vec!(
        String::from("Falling over"),
        String::from("First steps"),
        String::from("Ignored"),
        String::from("Not much to write home about"),
        String::from("Scream into the mic"),
        String::from("The Second Step"),
    )
}

fn get_playlist_source() -> korama::Playlist {
    let example_track_1 = korama::Track {
        track_name: String::from("First track"),
        artist: String::from("Example artist"),
        album: String::from("Example album"),
        track_number: String::from(""),
        path: String::from("/some/path"),
    };
    let example_track_2 = korama::Track {
        track_name: String::from("Second track"),
        artist: String::from("Extrample artist"),
        album: String::from("Extrample album"),
        track_number: String::from("4"),
        path: String::from("/some/other/path"),
    };
    let example_track_3 = korama::Track {
        track_name: String::from("Third track"),
        artist: String::from("Nextrample artist"),
        album: String::from("Nextrample album"),
        track_number: String::from("4.2"),
        path: String::from("/some/other/path/again"),
    };

    let mut playlist = korama::Playlist::new(String::from("Test source playlist"));

    let example_tracks = vec!(example_track_1, example_track_2, example_track_3);

    playlist.add_track(example_tracks[0].clone());
    playlist.add_track(example_tracks[1].clone());
    playlist.add_track(example_tracks[2].clone());

    playlist
}

fn get_library_source() -> korama::MusicLibrary {
    let mut test_library_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_library_path.push("resources/test/library");

    let mut library = korama::MusicLibrary::new(
        String::from("Test source library"),
        test_library_path.to_str().unwrap().to_string(),
    );

    library.scan();

    library
}
