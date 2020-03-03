use std::path::PathBuf;
use korama;

#[test]
fn create_library() {
    let library_name = String::from("My library");
    let library_path = String::from(".");
    let library = korama::MusicLibrary::new(library_name.clone(), library_path.clone());

    assert!(library.get_name() == library_name);
    assert!(library.get_path() == library_path);
}

#[test]
fn test_save_and_load() {
    // TODO: Remove saved test libraries
    let mut saved_library_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    saved_library_path.push("resources/test/library/saved_libraries");
    
    {
        let mut library = set_up_test_library();
        library.scan();
        // Ensure we have the expected contents before we save
        check_tracks_in_library_by_artist_and_album(library);
        library.save(saved_library_path.to_str().unwrap().to_string());
    }

    // TODO: Check the created file is as expected (for which we need to know how we're saving it)

    let mut library = korama::MusicLibrary::load(saved_library_path, String::from("Test library"));
    check_tracks_in_library_by_artist_and_album(library);

    // TODO: Remove saved test libraries
}

#[test]
fn get_tracks_by_artist_and_album() {
    let mut library = set_up_test_library();

    library.scan();

    check_tracks_in_library_by_artist_and_album(library);
}

#[test]
fn get_tracks_by_track_name() {
    let mut library = set_up_test_library();

    library.scan();

    let expected = vec![
        korama::Track{
            track_name: String::from("Falling over"),
            artist: String::from("Another artist"),
            album: String::from("The Ignored And the Found"),
            track_number: String::from("2.1"),
            path: get_full_track_path(String::from("another_artist/good_album/hidden_track.mp3")),
        },
        korama::Track{
            track_name: String::from("First steps"),
            artist: String::from("A different somebody"),
            album: String::from("Ignored"),
            track_number: String::from("1"),
            path: get_full_track_path(String::from("artist2/live_cover.mp3")),
        },
        korama::Track{
            track_name: String::from("First steps"),
            artist: String::from("Another artist"),
            album: String::from("The Ignored And the Found"),
            track_number: String::from("2"),
            path: get_full_track_path(String::from("another_artist/good_album/another_track.mp3")),
        },
        korama::Track{
            track_name: String::from("Ignored"),
            artist: String::from("Ignored"),
            album: String::from("Ignored"),
            track_number: String::from("1"),
            path: get_full_track_path(String::from("ignored.mp3")),
        },
        korama::Track{
            track_name: String::from("Not much to write home about"),
            artist: String::from("A different somebody"),
            album: String::from("The Greatest Album of Negligible MP3s"),
            track_number: String::from("1"),
            path: get_full_track_path(String::from("artist2/album/ignored.mp3")),
        },
        korama::Track{
            track_name: String::from("Scream into the mic"),
            artist: String::from("Somebody"),
            album: String::from("Live Bootleg"),
            track_number: String::from(""),
            path: get_full_track_path(String::from("artist1/test.mp3")),
        },
        korama::Track{
            track_name: String::from("The Second Step"),
            artist: String::from("Another artist"),
            album: String::from("The Ignored And the Found"),
            track_number: String::from("1"),
            path: get_full_track_path(String::from("another_artist/good_album/first_track.mp3")),
        },
    ];

    let result = library.get_tracks_by_title();

    assert!(result == expected,
            "Results not as expected.\nResults were:\n{}\nExpected:\n{}",
            generate_track_output(result),
            generate_track_output(expected),
            );
}

fn check_tracks_in_library_by_artist_and_album(library: korama::MusicLibrary) {
    let expected = vec![
        korama::Track{
            track_name: String::from("First steps"),
            artist: String::from("A different somebody"),
            album: String::from("Ignored"),
            track_number: String::from("1"),
            path: get_full_track_path(String::from("artist2/live_cover.mp3")),
        },
        korama::Track{
            track_name: String::from("Not much to write home about"),
            artist: String::from("A different somebody"),
            album: String::from("The Greatest Album of Negligible MP3s"),
            track_number: String::from("1"),
            path: get_full_track_path(String::from("artist2/album/ignored.mp3")),
        },
        korama::Track{
            track_name: String::from("The Second Step"),
            artist: String::from("Another artist"),
            album: String::from("The Ignored And the Found"),
            track_number: String::from("1"),
            path: get_full_track_path(String::from("another_artist/good_album/first_track.mp3")),
        },
        korama::Track{
            track_name: String::from("First steps"),
            artist: String::from("Another artist"),
            album: String::from("The Ignored And the Found"),
            track_number: String::from("2"),
            path: get_full_track_path(String::from("another_artist/good_album/another_track.mp3")),
        },
        korama::Track{
            track_name: String::from("Falling over"),
            artist: String::from("Another artist"),
            album: String::from("The Ignored And the Found"),
            track_number: String::from("2.1"),
            path: get_full_track_path(String::from("another_artist/good_album/hidden_track.mp3")),
        },
        korama::Track{
            track_name: String::from("Ignored"),
            artist: String::from("Ignored"),
            album: String::from("Ignored"),
            track_number: String::from("1"),
            path: get_full_track_path(String::from("ignored.mp3")),
        },
        korama::Track{
            track_name: String::from("Scream into the mic"),
            artist: String::from("Somebody"),
            album: String::from("Live Bootleg"),
            track_number: String::from(""),
            path: get_full_track_path(String::from("artist1/test.mp3")),
        },
    ];

    let result = library.get_tracks_by_artist_and_album();

    assert!(result == expected,
            "Results not as expected.\nResults were:\n{}\nExpected:\n{}",
            generate_track_output(result),
            generate_track_output(expected),
            );
}

fn generate_track_output(tracks: Vec<korama::Track>) -> String {
    let mut output = String::from("");
    output.push_str("Found ");
    output.push_str(&tracks.len().to_string());
    output.push_str(" tracks.\n");
    for track in tracks {
        output.push_str("Artist: ");
        output.push_str(&track.artist);
        output.push_str(", Album: ");
        output.push_str(&track.album);
        output.push_str(", Track: ");
        output.push_str(&track.track_name);
        output.push_str(", Track number: ");
        output.push_str(&track.track_number);
        output.push_str(", Path: ");
        output.push_str(&track.path);
        output.push_str("\n");
    }
    output
}

fn get_full_track_path(rel_path: String) -> String {
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.push("resources/test/library");
    file_path.push(rel_path);
    file_path.to_str().unwrap().to_string()
}

fn set_up_test_library() -> korama::MusicLibrary {
    let mut test_library_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_library_path.push("resources/test/library");

    korama::MusicLibrary::new(
        String::from("Test library"),
        test_library_path.to_str().unwrap().to_string(),
    )
}
