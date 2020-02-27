use std::path::PathBuf;
use korama;

#[test]
fn create_library() {
    let library_name = String::from("My library");
    // Use empty device name as nothing is expected to be done with the path until instructed to scan.
    let library_path = String::from("fake_path");
    let library = korama::MusicLibrary::new(library_name.clone(), library_path.clone());

    assert!(library.get_name() == library_name);
    assert!(library.get_path() == library_path);
}

#[test]
fn get_tracks_by_track_name() {
    let library = set_up_test_library();

    library.scan()

    let expected = vec![
        korama::Track{
            String::from("Falling over"),
            String::from("Another artist"),
            String::from("The Ignored And the Found"),
            String::from("2"),
            String::from("another_artist/good_album/hidden_track.mp3"),
        },
        korama::Track{
            String::from("First steps"),
            String::from("A different somebody"),
            String::from(""),
            String::from(""),
            String::from("artist2/live_cover.mp3"),
        },
        korama::Track{
            String::from("First steps"),
            String::from("Another artist"),
            String::from("The Ignored And the Found"),
            String::from("2"),
            String::from("another_artist/good_album/another_track.mp3"),
        },
        korama::Track{
            String::from("Not much to write home about"),
            String::from("A different somebody"),
            String::from("The Greatest Album of Negligible MP3s"),
            String::from("1"),
            String::from("artist2/album/ignored.mp3"),
        },
        korama::Track{
            String::from("Scream into the mic"),
            String::from("Somebody"),
            String::from("Live Bootleg"),
            String::from(""),
            String::from("artist1/test.mp3"),
        },
        korama::Track{
            String::from("The Second Step"),
            String::from("Another artist"),
            String::from("The Ignored And the Found"),
            String::from("1"),
            String::from("another_artist/good_album/first_track.mp3"),
        },
    ]

    let result = library.get_tracks_by_title()

    assert!(result == expected)
}


fn set_up_test_library() -> korama::MusicLibrary {
    let mut test_library_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_library_path.push("resources/test/library");

    korama::MusicLibrary::new(
        String::from("Test library"),
        test_library_path.to_str().unwrap().to_string(),
    )
}
