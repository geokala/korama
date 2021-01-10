use crate::playlist::Playlist;
use crate::track::Track;
use std::fs::File;
use std::io::BufReader;
use rodio::Sink;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;


pub struct Queue {
    playlist: Option<Playlist>,
    history: Vec<Track>,
    player_controller: Option<mpsc::Sender<String>>,
    playlist_controller: Option<mpsc::Sender<Playlist>>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue{
            playlist: None,
            history: Vec::new(),
            player_controller: None,
            playlist_controller: None,
        }
    }

    pub fn use_playlist(&mut self, playlist: Playlist) {
        self.playlist = Some(playlist.clone());
        self.get_playlist_controller().send(playlist);
    }

    pub fn get_playlist(self) -> Option<Playlist> {
        self.playlist
    }

    fn get_playlist_controller(&mut self) -> &mpsc::Sender<Playlist> {
         match self.playlist_controller {
             Some(_) => (),
             None => self.create_player(),
         };
         self.playlist_controller.as_ref().unwrap()
    }

    fn get_controller(&mut self) -> &mpsc::Sender<String> {
         match self.player_controller {
             Some(_) => (),
             None => self.create_player(),
         };
         self.player_controller.as_ref().unwrap()
    }

    fn create_player(&mut self) {
         let (sender, receiver) = mpsc::channel();
         let (playlist_sender, playlist_receiver) = mpsc::channel();
         thread::spawn(move || {
             let device = rodio::default_output_device().unwrap();
             let sink = Sink::new(&device);
             let mut playing = false;
             let mut playlist = Playlist::new(String::from("Empty"));
             loop {
                 let received = receiver.try_recv();
                 match received {
                     Ok(msg) => {
                         if msg == String::from("play") {
                             playing = true;
                         };
                     },
                     Err(_) => (),
                 };

                 let received_playlist = playlist_receiver.try_recv();
                 match received_playlist {
                     Ok(msg) => {
                         playlist = msg;
                     },
                     Err(_) => (),
                 };

                 if !playing {
                     thread::sleep(Duration::from_millis(50));
                     continue;
                 };
                 if sink.empty() {
                     // TODO: Where self is used here it needs to not be, somehow
                     let next_track = playlist.next();
                     //let next_track: std::option::Option<String> = None;
                     match next_track {
                         Some(file) => {
                             //self.history.push(track.clone());
                             let file = File::open("test.txt").unwrap();
                             let source = match rodio::Decoder::new(BufReader::new(file)) {
                                 Ok(src) => src,
                                 // TODO: This should be logging, not panicking.
                                 //Err(err) => panic!("Could not play file: {}: {:#?}", &track.path, err),
                                 Err(err) => panic!("Sad time"),
                             };
                             sink.append(source);
                             sink.play();
                         },
                         None => thread::sleep(Duration::from_millis(50)),
                     };
                 } else {
                     // There is a track playing, wait
                     thread::sleep(Duration::from_millis(50));
                 }
             };
         });
         self.player_controller = Some(sender);
         self.playlist_controller = Some(playlist_sender);
    }

    pub fn play(&mut self) {
        self.get_controller().send(String::from("play"));
    }

    pub fn get_history(&self) -> Vec<Track> {
        self.history.clone()
    }
}
