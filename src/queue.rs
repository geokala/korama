use crate::playlist::Playlist;
use crate::track::Track;
use std::fs::File;
use std::io::BufReader;
use rodio::Sink;
use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;

#[derive(PartialEq)]
#[derive(PartialOrd)]
pub enum QueueAction {
    Stopped,
    Stopping,
    Preparing,
    Playing,
}

struct QueueState {
    pub current_track: Option<Track>,
    pub action: QueueAction,
}

pub struct Queue {
    playlist: Arc<Mutex<Option<Playlist>>>,
    history: Arc<Mutex<Vec<Track>>>,
    state: Arc<Mutex<QueueState>>,
    player_created: bool,
}

impl Queue {
    pub fn new() -> Queue {
        Queue{
            playlist: Arc::new(Mutex::new(None)),
            history: Arc::new(Mutex::new(Vec::new())),
            state: Arc::new(Mutex::new(QueueState {
                current_track: None,
                action: QueueAction::Stopped,
            })),
            player_created: false,
        }
    }

    pub fn use_playlist(&mut self, playlist: Playlist) {
        self.playlist = Arc::new(Mutex::new(Some(playlist)));
    }

    pub fn get_playlist(self) -> Option<Playlist> {
        self.playlist.lock().unwrap().clone()
    }

    fn create_player(&mut self) {
         let playlist = self.playlist.clone();
         let history = self.history.clone();
         let state = self.state.clone();
         thread::spawn(move || {
             let device = rodio::default_output_device().unwrap();
             let sink = Sink::new(&device);
             loop {
                 if state.lock().unwrap().action <= QueueAction::Stopping {
                     thread::sleep(Duration::from_millis(50));
                     if state.lock().unwrap().action != QueueAction::Stopped {
                         state.lock().unwrap().action = QueueAction::Stopped;
                     };
                     continue;
                 };
                 if sink.empty() {
                     if state.lock().unwrap().action != QueueAction::Playing {
                         state.lock().unwrap().action = QueueAction::Playing;
                     };
                     let next_track = playlist.lock().unwrap().as_mut().unwrap().next();
                     match next_track {
                         Some(track) => {
                             state.lock().unwrap().current_track = Some(track.clone());
                             history.lock().unwrap().push(track.clone());
                             let file = File::open(track.path).unwrap();
                             let source = match rodio::Decoder::new(BufReader::new(file)) {
                                 Ok(src) => src,
                                 // TODO: This should be logging, not panicking.
                                 //Err(err) => panic!("Could not play file: {}: {:#?}", &track.path, err),
                                 Err(_) => panic!("Sad time"),
                             };
                             sink.append(source);
                             sink.play();
                         },
                         None => {
                             state.lock().unwrap().current_track = None;
                             state.lock().unwrap().action = QueueAction::Stopped;
                             thread::sleep(Duration::from_millis(50));
                         },
                     };
                 } else {
                     // There is a track playing, wait
                     thread::sleep(Duration::from_millis(50));
                 }
             };
         });
    }

    pub fn play(&mut self) {
        if !self.player_created {
            self.create_player();
        };
        self.state.lock().unwrap().action = QueueAction::Preparing;
    }

    pub fn is_playing(&self) -> bool {
        return self.state.lock().unwrap().action >= QueueAction::Preparing;
    }

    pub fn get_history(&self) -> Vec<Track> {
        self.history.lock().unwrap().clone()
    }

    pub fn set_history(&self, new_history: Vec<Track>) {
        std::mem::replace(&mut *self.history.lock().unwrap(), new_history);
    }
}
