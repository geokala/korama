use korama;
use korama::Saveable;


#[test]
fn set_queue_playlist() {
    let playlist = korama::Playlist::new(String::from("Test playlist for queue"));

    let mut queue = korama::Queue::new();

    queue.use_playlist(playlist);

    assert_eq!(queue.get_playlist().unwrap().get_name(), String::from("Test playlist for queue"));
}
