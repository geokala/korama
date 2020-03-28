use std::path::PathBuf;
use korama;
use korama::Saveable;


#[test]
fn add_dynamic_source_playlist() {
    let mut dyn_playlist = korama::Playlist::new(String::from("Test dynamic playlist"));

    let source_playlist = get_playlist_source();

    dyn_playlist.add_dynamic_playlist_source(source_playlist.clone());

    let dynamic_playlist_sources = dyn_playlist.get_dynamic_playlist_sources();
    let dynamic_library_sources = dyn_playlist.get_dynamic_library_sources();

    assert_eq!(dynamic_playlist_sources.len(), 1);
    assert_eq!(dynamic_playlist_sources[0].get_name(), source_playlist.get_name());
    assert_eq!(dynamic_library_sources.len(), 0);

    let track = dyn_playlist.next().unwrap();

    assert!(get_playlist_titles().contains(&track.track_name));
}

#[test]
fn add_dynamic_source_library() {
    let mut dyn_playlist = korama::Playlist::new(String::from("Test dynamic playlist"));

    let source_library = get_library_source();

    dyn_playlist.add_dynamic_library_source(source_library.clone());

    let dynamic_playlist_sources = dyn_playlist.get_dynamic_playlist_sources();
    let dynamic_library_sources = dyn_playlist.get_dynamic_library_sources();

    assert_eq!(dynamic_playlist_sources.len(), 0);
    assert_eq!(dynamic_library_sources.len(), 1);
    assert_eq!(dynamic_library_sources[0].get_name(), source_library.get_name());

    let track = dyn_playlist.next().unwrap();

    assert!(get_library_titles().contains(&track.track_name));
}

#[test]
fn add_dynamic_source_library_and_playlist() {
    let mut dyn_playlist = korama::Playlist::new(String::from("Test dynamic playlist"));

    let source_playlist = get_playlist_source();
    let source_library = get_library_source();

    dyn_playlist.add_dynamic_playlist_source(source_playlist.clone());
    dyn_playlist.add_dynamic_library_source(source_library.clone());

    let dynamic_playlist_sources = dyn_playlist.get_dynamic_playlist_sources();
    let dynamic_library_sources = dyn_playlist.get_dynamic_library_sources();

    assert_eq!(dynamic_playlist_sources.len(), 1);
    assert_eq!(dynamic_playlist_sources[0].get_name(), source_playlist.get_name());
    assert_eq!(dynamic_library_sources.len(), 1);
    assert_eq!(dynamic_library_sources[0].get_name(), source_library.get_name());

    let track = dyn_playlist.next().unwrap();

    let mut possible_names = get_playlist_titles();
    possible_names.append(&mut get_library_titles());

    assert!(possible_names.contains(&track.track_name));
}

#[test]
fn test_dynamic_window_small() {
    let mut dyn_playlist = korama::Playlist::new(String::from("Test dynamic playlist"));

    let source_playlist = get_playlist_source();

    dyn_playlist.add_dynamic_playlist_source(source_playlist.clone());

    // Generate some tracks
    let mut titles = Vec::new();
    for _ in 1..1000 {
        let track = dyn_playlist.next().unwrap();
        titles.push(track.track_name.clone());
    };

    for title in get_playlist_titles() {
        let mut indices = Vec::new();

        for (i, dyn_title) in titles.iter().enumerate() {
            if dyn_title == &title {
                indices.push(i);
            };
        };

        let mut prev_index;
        if indices.len() > 0 {
            prev_index = indices.remove(0);
        } else {
            break;
        };
        for index in indices {
            // We expect the dynamic window to have a size of at least the sum of
            // the dynamic source lengths / 2, rounded up, -1
            // e.g. 3 or 4 tracks in all dynamic sources -> window size 1
            // e.g. 15 or 16 tracks in all dynamic sources -> window size 7
            // A maximum window size would be nice to have but horrible to test
            assert!(index - prev_index > 1);
            prev_index = index;
        };
    }
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
