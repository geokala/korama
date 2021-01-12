use crate::playlist::Playlist;
use crate::track::Track;
use std::fs::File;
use std::io::BufReader;
use rodio::Sink;
use std::sync::mpsc;
use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;


pub struct Queue {
    playlist: Arc<Mutex<Option<Playlist>>>,
    history: Arc<Mutex<Vec<Track>>>,
    player_controller: Option<mpsc::Sender<String>>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue{
            playlist: Arc::new(Mutex::new(None)),
            history: Arc::new(Mutex::new(Vec::new())),
            player_controller: None,
        }
    }

    pub fn use_playlist(&mut self, playlist: Playlist) {
        self.playlist = Arc::new(Mutex::new(Some(playlist)));
    }

    pub fn get_playlist(self) -> Option<Playlist> {
        self.playlist.lock().unwrap().clone()
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
         let playlist = self.playlist.clone();
         let history = self.history.clone();
         thread::spawn(move || {
             let device = rodio::default_output_device().unwrap();
             let sink = Sink::new(&device);
             let mut playing = false;
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

                 if !playing {
                     thread::sleep(Duration::from_millis(50));
                     continue;
                 };
                 if sink.empty() {
                     // TODO: Where self is used here it needs to not be, somehow
                     let next_track = playlist.lock().unwrap().as_mut().unwrap().next();
                     match next_track {
                         Some(track) => {
                             history.lock().unwrap().push(track.clone());
                             let file = File::open(track.path).unwrap();
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
    }

    pub fn play(&mut self) {
        self.get_controller().send(String::from("play"));
    }

    pub fn get_history(&self) -> Vec<Track> {
        self.history.lock().unwrap().clone()
    }
}
