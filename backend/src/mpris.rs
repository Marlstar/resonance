use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError, TrySendError};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use mpris_server::Player;
use crate::Song;

pub struct Mpris {
    tx: Sender<Emit>,
    rx: Receiver<Recv>,
    _handle: JoinHandle<()>,
}
impl Mpris {
    pub fn new() -> Self {
        let (etx, erx) = channel::<Emit>();
        let (rtx, rrx) = channel::<Recv>();
        let handler = MprisHandler::new(rtx, erx);
        let _handle = handler.run();

        return Self { tx: etx, rx: rrx, _handle };
    }
}
impl Default for Mpris {
    fn default() -> Self {
        Self::new()
    }
}
impl Mpris {
    pub fn recv(&self) -> Option<Recv> {
        match self.rx.try_recv() {
            Ok(a) => Some(a),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => panic!("mpris channel disconnected"),
        }
    }

    fn send(&self, msg: Emit) {
        match self.tx.send(msg) {
            Ok(()) => {},
            Err(_) => panic!("mpris channel disconnected"),
            //Err(TrySendError::Full(_)) => eprintln!("mpris emit channel full"),
        }
    }

    pub fn resume(&self) {
        self.send(Emit::Play)
    }

    pub fn pause(&self) {
        self.send(Emit::Pause)
    }

    pub fn play_song(&self, s: Song) {
        self.send(Emit::Song(s))
    }
}

// https://docs.rs/mpris-server/0.8.1/mpris_server/trait.PlayerInterface.html
pub struct MprisHandler {
    tx: Sender<Recv>,
    rx: Receiver<Emit>,
}
impl MprisHandler {
    pub fn new(tx: Sender<Recv>, rx: Receiver<Emit>) -> Self {
        Self { tx, rx }
    }

    pub fn run(self) -> JoinHandle<()> {
        //thread::spawn(move || futures::executor::block_on(self.main()))
        thread::spawn(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(self.main());
        })
    }

    pub async fn main(self) {
        let mpris = mpris_server::Player::builder("resonance")
            .can_play(true)
            .can_pause(true)
            .can_seek(false)
            .build().await.unwrap();

        let tx = self.tx.clone();
        //let f = move |_| tx.send(Recv::PlayPause).unwrap();
        //mpris.connect_play_pause(self.sender(Recv::PlayPause));
        mpris.connect_play_pause(move |_| tx.send(Recv::PlayPause).unwrap());

        println!("started mpris server");

        self.main_loop(mpris).await;
    }
    async fn main_loop(self, mpris: Player) {
        let l = async {
            loop {
                'to_mpris: {
                    //let msg = match self.rx.try_recv() {
                    //    Ok(a) => a,
                    //    Err(TryRecvError::Disconnected) => panic!("mpris channel disconnected"),
                    //    Err(TryRecvError::Empty) => break 'to_mpris,
                    //};
                    let msg = match self.rx.try_recv() {
                        Ok(a) => a,
                        Err(_) => break 'to_mpris,
                    };

                    match msg {
                        Emit::Play => Self::play(&mpris).await,
                        Emit::Pause => Self::pause(&mpris).await,
                        Emit::Song(s) => Self::song(&mpris, s).await,
                    };
                }

                'to_resonance: {
                }

                tokio::time::sleep(Duration::from_millis(50)).await;
            }
        };

        tokio::join!(mpris.run(), l);
    }

    async fn play(mpris: &Player) {
        println!("play");
        let _ = mpris.set_playback_status(mpris_server::PlaybackStatus::Playing).await;
    }
    async fn pause(mpris: &Player) {
        let _ = mpris.set_playback_status(mpris_server::PlaybackStatus::Paused).await;
    }

    async fn song(mpris: &Player, song: crate::Song) {
        let mut metadata = mpris.metadata().clone();
        metadata.set_title(Some(&song.name));
        metadata.set_artist(Some([&song.author]));
        metadata.set_album(Some(&song.album));
        metadata.set_art_url(Some(format!("file://{}", crate::dirs().song_thumbnail(&song.ytid).display())));

        let _ = mpris.set_metadata(metadata).await;
        Self::play(mpris).await;
    }
}
impl MprisHandler {
    fn send(&self, msg: Recv) {
        match self.tx.send(msg) {
            Ok(()) => {},
            Err(_) => panic!("mpris channel disconnected"),
            //Err(TrySendError::Full(_)) => eprintln!("mpris emit channel full"),
        }
    }

    fn sender(&self, msg: Recv) -> impl Fn(&Player) {
        // TODO: error handling
        let tx = self.tx.clone();
        move |_| tx.send(msg.clone()).unwrap()
    }
}

#[derive(Debug, Clone)]
pub enum Emit {
    Play,
    Pause,
    Song(crate::Song),
}

#[derive(Debug, Clone)]
pub enum Recv {
    Play,
    Pause,
    PlayPause,
}
