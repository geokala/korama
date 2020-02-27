use korama;

#[test]
fn create_library() {
    let library_name = String::from("My library");
    // Use empty device name as nothing is expected to be done with the path until instructed to scan.
    let library_path = String::from("fake_path");
    let test_library = korama::MusicLibrary::new(library_name.clone(), library_path.clone());

    assert!(test_library.get_name() == library_name);
    assert!(test_library.get_path() == library_path);
}
