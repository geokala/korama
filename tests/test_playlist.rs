use korama;

#[test]
fn create_playlist() {
    let playlist_name = String::from("My playlist");

    let playlist = korama::Playlist::new(playlist_name.clone());

    assert!(playlist.get_name() == playlist_name);
}

#[test]
fn add_and_delete_tracks() {
    let mut playlist = korama::Playlist::new(String::from("Test playlist"));

    let example_track_1 = korama::Track {
        track_name: String::from("Example track"),
        artist: String::from("Example artist"),
        album: String::from("Example album"),
        track_number: String::from(""),
        path: String::from("/some/path"),
    };
    let example_track_2 = korama::Track {
        track_name: String::from("Extrample track"),
        artist: String::from("Extrample artist"),
        album: String::from("Extrample album"),
        track_number: String::from("4"),
        path: String::from("/some/other/path"),
    };
    let example_track_3 = korama::Track {
        track_name: String::from("Nextrample track"),
        artist: String::from("Nextrample artist"),
        album: String::from("Nextrample album"),
        track_number: String::from("4.2"),
        path: String::from("/some/other/path/again"),
    };

    playlist.add_track(example_track_1.clone());
    playlist.add_track(example_track_2.clone());
    playlist.add_track(example_track_3.clone());

    let expected = vec!(example_track_1.clone(), example_track_2.clone(), example_track_3.clone());

    assert!(playlist.get_tracks() == expected, "Add tracks failed.");

    playlist.remove_track(1);

    let expected = vec!(example_track_1, example_track_3);

    assert!(playlist.get_tracks() == expected, "Remove tracks failed.");
}
