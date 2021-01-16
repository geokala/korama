use std::path::PathBuf;
use korama;
use korama::Saveable;
use std::{thread,time};


#[test]
fn set_queue_playlist() {
    let playlist = korama::Playlist::new(String::from("Test playlist for queue"));

    let mut queue = korama::Queue::new();

    queue.use_playlist(playlist);

    assert_eq!(queue.get_playlist().unwrap().get_name(), String::from("Test playlist for queue"));
}

#[test]
fn set_history() {
    let queue = korama::Queue::new();

    let result = queue.get_history();
    let expected = vec![];

    assert!(result == expected,
            "Results not as expected.\nResults were:\n{}\nExpected:\n{}",
            generate_track_output(result),
            generate_track_output(expected),
            );

    let update = vec![
        korama::Track{
            track_name: String::from("Not a real track"),
            artist: String::from("Really not"),
            album: String::from("It doesn't exist"),
            track_number: String::from("e"),
            path: String::from("/not/real/at/all"),
        },
    ];

    queue.set_history(update.clone());

    let updated_result = queue.get_history();

    assert!(updated_result == update,
            "Results not as expected.\nResults were:\n{}\nExpected:\n{}",
            generate_track_output(updated_result),
            generate_track_output(update),
            );
}

#[test]
fn play_tracks() {
    let mut library = set_up_test_library();
    library.scan();

    let mut playlist = korama::Playlist::new(String::from("Test playlist for queue"));
    for track in library.get_tracks_by_title() {
        playlist.add_track(track.clone());
    };

    let mut queue = korama::Queue::new();
    queue.use_playlist(playlist);

    queue.play();

    while queue.is_playing() {
      thread::sleep(time::Duration::from_millis(50));
    };

    let result = queue.get_history();

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
