use crate::playlist::Playlist;
use crate::track::Track;
use std::fs::File;
use std::io::BufReader;
use rodio::Sink;
use std::sync::mpsc;
use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;

#[derive(PartialEq)]
pub enum QueueActivity {
    Stopped,
    Playing,
}

pub enum QueueAction {
    SkipForward,
    SkipBack,
}

struct QueueState {
    pub current_track: Option<Track>,
    pub action: QueueActivity,
}

pub struct Queue {
    playlist: Arc<Mutex<Option<Playlist>>>,
    history: Arc<Mutex<Vec<Track>>>,
    state: Arc<Mutex<QueueState>>,
    player_controller: Option<mpsc::Sender<QueueAction>>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue{
            playlist: Arc::new(Mutex::new(None)),
            history: Arc::new(Mutex::new(Vec::new())),
            state: Arc::new(Mutex::new(QueueState {
                current_track: None,
                action: QueueActivity::Stopped,
            })),
            player_controller: None,
        }
    }

    pub fn use_playlist(&mut self, playlist: Playlist) {
        self.playlist = Arc::new(Mutex::new(Some(playlist)));
    }

    pub fn get_playlist(self) -> Option<Playlist> {
        self.playlist.lock().unwrap().clone()
    }

    fn get_controller(&mut self) -> &mpsc::Sender<QueueAction> {
         match self.player_controller {
             Some(_) => (),
             None => self.create_player(),
         };
         self.player_controller.as_ref().unwrap()
    }

    fn create_player(&mut self) {
         match self.player_controller {
             Some(_) => return,
             None => (),
         };
         let playlist = self.playlist.clone();
         let history = self.history.clone();
         let state = self.state.clone();
         let (sender, receiver) = mpsc::channel();
         thread::spawn(move || {
             let device = rodio::default_output_device().unwrap();
             let sink = Sink::new(&device);
             loop {
                 let received = receiver.try_recv();
                 let new_sink = match received {
                     Ok(msg) => {
                         if msg == QueueAction::skip() {
                             sink.drop();
                             Sink::new(&device)
                         };
                     },
                     Err(_) => (),
                 };

                 if state.lock().unwrap().action == QueueActivity::Stopped {
                     thread::sleep(Duration::from_millis(50));
                     continue;
                 };

                 if sink.empty() {

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
                             state.lock().unwrap().action = QueueActivity::Stopped;
                             thread::sleep(Duration::from_millis(50));
                         },
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
        self.create_player();
        self.state.lock().unwrap().action = QueueActivity::Playing;
    }

    pub fn skip_forward(&mut self) {
        self.get_controller().send(QueueAction::SkipForward);
    }

    pub fn skip_back(&mut self) {
        self.get_controller().send(QueueAction::SkipBack);
    }

    pub fn is_playing(&self) -> bool {
        return self.state.lock().unwrap().action == QueueActivity::Playing;
    }

    pub fn get_history(&self) -> Vec<Track> {
        self.history.lock().unwrap().clone()
    }

    pub fn set_history(&self, new_history: Vec<Track>) {
        std::mem::replace(&mut *self.history.lock().unwrap(), new_history);
    }
}
