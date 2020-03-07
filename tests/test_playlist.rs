use korama;

#[test]
fn create_playlist() {
    let playlist_name = String::from("My playlist");

    let playlist = korama::Playlist::new(playlist_name.clone());

    assert!(playlist.get_name() == playlist_name);
}
