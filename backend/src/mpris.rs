use std::sync::mpsc::{sync_channel, SyncSender, Receiver, TryRecvError, TrySendError};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use mpris_server::Player;
use crate::Song;

pub struct Mpris {
    tx: SyncSender<Emit>,
    rx: Receiver<Recv>,
    _handle: JoinHandle<()>,
}
impl Mpris {
    pub fn new() -> Self {
        let (etx, erx) = sync_channel::<Emit>(5);
        let (rtx, rrx) = sync_channel::<Recv>(5);
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
    fn send(&self, msg: Emit) {
        match self.tx.try_send(msg) {
            Ok(()) => {},
            Err(TrySendError::Disconnected(_)) => panic!("mpris channel disconnected"),
            Err(TrySendError::Full(_)) => eprintln!("mpris emit channel full"),
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
    tx: SyncSender<Recv>,
    rx: Receiver<Emit>,
}
impl MprisHandler {
    pub fn new(tx: SyncSender<Recv>, rx: Receiver<Emit>) -> Self {
        Self { tx, rx }
    }

    pub fn run(self) -> JoinHandle<()> {
        thread::spawn(move || futures::executor::block_on(self.main()))
    }

    pub async fn main(self) {
        let mut mpris = mpris_server::Player::builder("resonance")
            .can_play(true)
            .can_pause(true)
            .can_seek(false)
            .build().await.unwrap();

        println!("started mpris server");

        futures::join!(
            mpris.run(),
            self.main_loop(&mut mpris)
        );
    }
    async fn main_loop(&self, mpris: &mut Player) {
        //mpris.set_volume(50.0);
        //println!("volume set");
        //let mut m = mpris.metadata().clone();
        //m.set_title(Some("test"));
        //let _ = mpris.set_metadata(m).await;
        //return;
        loop {
            'to_mpris: {
                //let msg = match self.rx.try_recv() {
                //    Ok(a) => a,
                //    Err(TryRecvError::Disconnected) => panic!("mpris channel disconnected"),
                //    Err(TryRecvError::Empty) => break 'to_mpris,
                //};
                let msg = match self.rx.recv_timeout(Duration::from_secs(1)) {
                    Ok(a) => a,
                    Err(_) => break 'to_mpris,
                };

                match msg {
                    Emit::Play => Self::play(mpris).await,
                    Emit::Pause => Self::pause(mpris).await,
                    Emit::Song(s) => Self::song(mpris, s).await,
                }
            }

            'to_resonance: {

            }

            std::thread::sleep(Duration::from_millis(500));
        }
    }

    async fn play(mpris: &mut Player) {
        println!("play");
        let _ = mpris.set_playback_status(mpris_server::PlaybackStatus::Playing).await;
    }
    async fn pause(mpris: &mut Player) {
        let _ = mpris.set_playback_status(mpris_server::PlaybackStatus::Paused).await;
    }

    async fn song(mpris: &mut Player, song: crate::Song) {
        let mut metadata = mpris.metadata().clone();
        metadata.set_title(Some(&song.name));
        metadata.set_artist(Some([&song.author]));
        metadata.set_album(Some(&song.album));

        let _ = mpris.set_metadata(metadata).await;
    }
}

pub enum Emit {
    Play,
    Pause,
    Song(crate::Song),
}

pub enum Recv {
    Play,
    Pause,
    PlayPause,
}
