use korama;

#[test]
fn create_playlist() {
    let playlist_name = String::from("My playlist");

    let playlist = korama::Playlist::new(playlist_name.clone());

    assert!(playlist.get_name() == playlist_name);
}

#[test]
fn add_and_delete_tracks_in_playlist() {
    let mut playlist = korama::Playlist::new(String::from("Test playlist"));

    let example_tracks = get_example_tracks();

    playlist.add_track(example_tracks[0].clone());
    playlist.add_track(example_tracks[1].clone());
    playlist.add_track(example_tracks[2].clone());

    assert!(&playlist.get_tracks() == &example_tracks, "Add tracks failed.");

    playlist.remove_track(1);

    let expected = vec!(example_tracks[0].clone(), example_tracks[2].clone());

    assert!(playlist.get_tracks() == expected, "Remove tracks failed.");
}

#[test]
fn step_through_playlist() {
    let mut playlist = korama::Playlist::new(String::from("Test playlist"));

    let example_tracks = get_example_tracks();

    playlist.add_track(example_tracks[0].clone());
    playlist.add_track(example_tracks[1].clone());
    playlist.add_track(example_tracks[2].clone());

    let mut next_track = playlist.next();
    match next_track {
        Some(track) => assert!(track == &example_tracks[0], "Failed starting playlist- found {}.", track.track_name),
        None => panic!("Failed starting playlist."),
    };

    next_track = playlist.next();
    match next_track {
        Some(track) => assert!(track == &example_tracks[1], "Failed stepping to second track- found {}.", track.track_name),
        None => panic!("Failed stepping to second track."),
    };

    next_track = playlist.next();
    match next_track {
        Some(track) => assert!(track == &example_tracks[2], "Failed stepping to last track- found {}.", track.track_name),
        None => panic!("Failed stepping to last track."),
    };

    next_track = playlist.next();
    match next_track {
        Some(track) => panic!("Failed finishing playlist- found {}.", track.track_name),
        None => println!("Found expected end of playlist."),
    };
}

#[test]
fn step_back_through_playlist() {
    let mut playlist = korama::Playlist::new(String::from("Test playlist"));

    let example_tracks = get_example_tracks();

    playlist.add_track(example_tracks[0].clone());
    playlist.add_track(example_tracks[1].clone());
    playlist.add_track(example_tracks[2].clone());

    playlist.next();
    playlist.next();
    playlist.next();
    playlist.next();

    let mut prev_track = playlist.prev();
    match prev_track {
        Some(track) => assert!(track == &example_tracks[2], "Failed first step back- found {}.", track.track_name),
        None => panic!("Failed first step back- found nothing."),
    };

    prev_track = playlist.prev();
    match prev_track {
        Some(track) => assert!(track == &example_tracks[1], "Failed second step back- found {}.", track.track_name),
        None => panic!("Failed second step back- found nothing."),
    };

    prev_track = playlist.prev();
    match prev_track {
        Some(track) => assert!(track == &example_tracks[0], "Failed third step back- found {}.", track.track_name),
        None => panic!("Failed third step back- found nothing."),
    };

    prev_track = playlist.prev();
    match prev_track {
        Some(track) => panic!("Failed returning to start of playlist- found {}.", track.track_name),
        None => println!("Found expected start of playlist."),
    };
}

#[test] fn step_too_far_through_playlist_then_back() {
    let mut playlist = korama::Playlist::new(String::from("Test playlist"));

    let example_tracks = get_example_tracks();

    playlist.add_track(example_tracks[0].clone());
    playlist.add_track(example_tracks[1].clone());
    playlist.add_track(example_tracks[2].clone());

    playlist.next();
    playlist.next();
    playlist.next();
    playlist.next();
    playlist.next();

    let prev_track = playlist.prev();
    match prev_track {
        Some(track) => assert!(track == &example_tracks[2], "Found unexpected track stepping back- found {}.", track.track_name),
        None => panic!("Playlist position ended somewhere unexpected."),
    };
}

#[test] fn step_back_from_start_of_playlist_then_forward() {
    let mut playlist = korama::Playlist::new(String::from("Test playlist"));

    let example_tracks = get_example_tracks();

    playlist.add_track(example_tracks[0].clone());
    playlist.add_track(example_tracks[1].clone());
    playlist.add_track(example_tracks[2].clone());

    playlist.prev();
    playlist.prev();

    let next_track = playlist.next();
    match next_track {
        Some(track) => assert!(track == &example_tracks[0], "Found unexpected track stepping forward- found {}.", track.track_name),
        None => panic!("Playlist position ended somewhere unexpected."),
    };
}

#[test] fn get_specific_tracks_from_playlist() {
    let mut playlist = korama::Playlist::new(String::from("Test playlist"));

    let example_tracks = get_example_tracks();

    playlist.add_track(example_tracks[0].clone());
    playlist.add_track(example_tracks[1].clone());
    playlist.add_track(example_tracks[2].clone());

    let mut track = playlist.get(0);
    match track {
        Some(track) => assert!(track == &example_tracks[0], "Failed getting first track- found {}.", track.track_name),
        None => panic!("Failed getting first track."),
    };

    track = playlist.get(1);
    match track {
        Some(track) => assert!(track == &example_tracks[1], "Failed getting second track- found {}.", track.track_name),
        None => panic!("Failed getting second track."),
    };

    track = playlist.get(2);
    match track {
        Some(track) => assert!(track == &example_tracks[2], "Failed getting last track- found {}.", track.track_name),
        None => panic!("Failed getting last track."),
    };

    track = playlist.get(42);
    match track {
        Some(track) => panic!("Found track where there should be none- found {}.", track.track_name),
        None => println!("Correctly found no track.."),
    };
}

fn get_example_tracks() -> Vec<korama::Track> {
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

    vec!(example_track_1, example_track_2, example_track_3)
}
