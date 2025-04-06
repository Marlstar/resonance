use mpris_server::{Player, Time, Metadata, TrackId};
use crate::Song;
use std::{thread::{self, JoinHandle}, time::Duration};
use tokio::sync::mpsc::{channel, Sender, Receiver};
use std::sync::OnceLock;
use tokio::sync::Mutex;

pub static CTX: OnceLock<Sender<Emit>> = OnceLock::new();
pub static ETX: OnceLock<Sender<Recv>> = OnceLock::new();
pub static CRX: OnceLock<Mutex<Receiver<Emit>>> = OnceLock::new();
pub static ERX: OnceLock<Mutex<Receiver<Recv>>> = OnceLock::new();

fn setup_channels() {
    let (ctx, crx) = channel::<Emit>(10);
    CTX.get_or_init(move || ctx);
    CRX.get_or_init(move || crx.into());

    let (etx, erx) = channel::<Recv>(10);
    ETX.get_or_init(move || etx);
    ERX.get_or_init(move || erx.into());
}

pub fn run() -> JoinHandle<()> {
    setup_channels();
    thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let joined = async || {
            //tokio::join!(mpris(), frontend_thread());
            mpris().await
        };
        rt.block_on(joined());
    })
}

async fn mpris() {
    let mpris = mpris_server::Player::builder("Resonance")
        .can_play(true)
        .can_pause(true)
        .can_seek(true)
        .build().await.unwrap();

    let mut met = mpris.metadata().clone();
    met.set_title(Some("Resonance"));
    let _ = mpris.set_metadata(met).await;

    let mut crx = CRX.get().unwrap().lock().await;

    let sender = |r: Recv| {
        let etx = ETX.get().unwrap();
        tokio::spawn(async {
            let _ = etx.send(r).await;
        });
    };

    //mpris.connect_play(move |_| { sender(Recv::Play) });
    //mpris.connect_pause(move |_| { sender(Recv::Pause) });
    mpris.connect_play_pause(move |_| { sender(Recv::PlayPause) });

    let l = async {
        loop {
            tokio::select! {
                received = crx.recv() => {
                    match received.expect("mpris channel disconnected") {
                        Emit::Play => Mpris::resume(&mpris).await,
                        Emit::Pause => Mpris::pause(&mpris).await,
                        Emit::Song(s) => Mpris::song(&mpris, s).await,
                        Emit::Seek(pos) => Mpris::seek(&mpris, pos).await,
                    }
                },
            }
        }
    };

    tokio::join!(mpris.run(), l);
}

struct Mpris;
impl Mpris {
    async fn resume(mpris: &Player) {
        let _ = mpris.set_playback_status(mpris_server::PlaybackStatus::Playing).await;
    }
    async fn pause(mpris: &Player) {
        let _ = mpris.set_playback_status(mpris_server::PlaybackStatus::Paused).await;
    }

    async fn song(mpris: &Player, song: crate::Song) {
        let mut metadata = mpris.metadata().clone(); metadata.set_title(Some(&song.name));
        let _ = mpris.set_metadata(Self::metadata(&song)).await;
        Self::resume(mpris).await;
    }

    fn metadata(song: &Song) -> Metadata {
        //let tid = mpris_server::zbus::zvariant::ObjectPath::try_from(crate::dirs().base().join(format!("tid_{}", &song.ytid)));
        let tids = format!("/TrackId/{}", &song.ytid.replace("-","_"));
        let tid = mpris_server::zbus::zvariant::ObjectPath::try_from(tids).unwrap();
        Metadata::builder()
            .title(&song.name)
            .artist([&song.author])
            .album(&song.album)
            .length(Time::from_secs(song.duration.into()))
            .art_url(format!("file://{}", crate::dirs().song_thumbnail(&song.ytid).display()))
            .trackid(tid)
            .build()
    }

    async fn seek(mpris: &Player, pos: f32) {
        let d = Time::from_secs(pos as i64);
        mpris.set_position(d);
    }
}

#[derive(Debug, Clone)]
pub enum Emit {
    Play,
    Pause,
    Song(Song),
    Seek(f32),
}

#[derive(Debug, Clone)]
pub enum Recv {
    Play,
    Pause,
    PlayPause,
}
