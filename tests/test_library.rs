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
fn get_tracks_by_track_name() {
    let mut library = set_up_test_library();

    library.scan();

    let expected = vec![
        korama::Track{
            track_name: String::from("Falling over"),
            artist: String::from("Another artist"),
            album: String::from("The Ignored And the Found"),
            track_number: String::from("2"),
            path: String::from("another_artist/good_album/hidden_track.mp3"),
        },
        korama::Track{
            track_name: String::from("First steps"),
            artist: String::from("A different somebody"),
            album: String::from(""),
            track_number: String::from(""),
            path: String::from("artist2/live_cover.mp3"),
        },
        korama::Track{
            track_name: String::from("First steps"),
            artist: String::from("Another artist"),
            album: String::from("The Ignored And the Found"),
            track_number: String::from("2"),
            path: String::from("another_artist/good_album/another_track.mp3"),
        },
        korama::Track{
            track_name: String::from("Not much to write home about"),
            artist: String::from("A different somebody"),
            album: String::from("The Greatest Album of Negligible MP3s"),
            track_number: String::from("1"),
            path: String::from("artist2/album/ignored.mp3"),
        },
        korama::Track{
            track_name: String::from("Scream into the mic"),
            artist: String::from("Somebody"),
            album: String::from("Live Bootleg"),
            track_number: String::from(""),
            path: String::from("artist1/test.mp3"),
        },
        korama::Track{
            track_name: String::from("The Second Step"),
            artist: String::from("Another artist"),
            album: String::from("The Ignored And the Found"),
            track_number: String::from("1"),
            path: String::from("another_artist/good_album/first_track.mp3"),
        },
    ];

    let result = library.get_tracks_by_title();

    assert!(result == &expected);
}


fn set_up_test_library() -> korama::MusicLibrary {
    let mut test_library_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_library_path.push("resources/test/library");

    korama::MusicLibrary::new(
        String::from("Test library"),
        test_library_path.to_str().unwrap().to_string(),
    )
}
