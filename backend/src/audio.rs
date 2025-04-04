use std::fs::File;
use std::thread::{self, JoinHandle, Thread};
use std::io::{BufReader, Read};
use std::sync::Arc;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TrySendError};
use std::time::Duration;
use crate::AM;
use orx_linked_list::DoublyList;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use crate::Song;
use crate::Error;
use crate::mpris::Emit;


type Queue = DoublyList<Song>;

pub struct AudioPlayer {
    // Audio thread
    _handle: JoinHandle<Result<(), Error>>,
    tx: SyncSender<Command>,
    rx: Receiver<Message>,

    // Mpris channel comms runtime
    rt: tokio::runtime::Runtime,

    // Main song control stuff
    pub playing: bool,
    queue: AM<Queue>,
    current_song: Option<Song>,
    position: f32,
    progress: f32,
    loop_type: AM<LoopType>,
}
impl AudioPlayer {
    pub fn new() -> Result<Self, Error> {
        let playing = false;
        let queue: AM<Queue> = AM::new(DoublyList::new());
        let current_song: Option<Song> = None;

        let (ctx, crx) = sync_channel::<Command>(3);
        // TODO: check whether 10 is enough
        let (mtx, mrx) = sync_channel::<Message>(10);

        let loop_type = AM::new(LoopType::None);

        let _handle = AudioHandler::run(mtx, crx);

        // TODO: error handling
        let rt = tokio::runtime::Runtime::new().unwrap();

        Ok(Self {
            _handle,
            tx: ctx, rx: mrx,
            rt,
            playing, queue, current_song,
            position: 0.0, progress: 0.0,
            loop_type,
        })
    }

    fn handle_message(&mut self, message: Message) {
        // TODO: in future, don't send position updates as often, estimate it instead and update occationally?
        match message {
            Message::Progress { percentage, seconds } => {
                self.progress = percentage;
                self.position = seconds;
            },
        }
    }

    pub fn current(&self) -> Option<Song> {
        return self.current_song.clone()
    }

    pub fn play_song(&mut self, song: Song) {
        self.current_song = Some(song.clone());
        self.send_command(Command::Play(song));
        self.playing = true;
    }

    pub fn pause(&mut self) {
        self.send_command(Command::Pause);
        self.playing = false;
    }
    pub fn resume(&mut self) {
        self.send_command(Command::Resume);
        self.playing = true;
    }

    fn send_command(&mut self, cmd: Command) {
        // Mpris
        let emit = match cmd {
            Command::Pause => Emit::Pause,
            Command::Resume => Emit::Play,
            Command::Play(ref s) => Emit::Song(s.clone()),
        };
        self.rt.spawn(async move {
            let _ = crate::mpris::CTX.get().unwrap().send(emit).await;
        });

        // Rodio
        match self.tx.try_send(cmd) {
            Ok(_) => {},
            Err(TrySendError::Full(_)) => eprintln!("command channel full"),
            Err(TrySendError::Disconnected(_)) => panic!("audio command channel disconnected"),
        };
    }
}

struct AudioHandler {
    // Rodio
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    sink: Sink,

    // Channel
    pub tx: SyncSender<Message>,
    pub rx: Receiver<Command>,

    // Audio stuff
    progress: f32,
    loop_type: AM<LoopType>,
    current: Option<Song>,

    // Audio settings
    volume: AM<f32>,
}
impl AudioHandler {
    pub fn new(tx: SyncSender<Message>, rx: Receiver<Command>) -> Result<Self, Error> {
        let (_stream, _stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&_stream_handle)?;
        
        let progress = 0f32;
        
        let volume = AM::new(0.8f32);
        let loop_type = AM::new(LoopType::None);
        let current = None;

        Ok(Self {
            _stream, _stream_handle, sink,
            tx, rx,
            progress, loop_type, current,
            volume,
        })
    }

    pub fn run(tx: SyncSender<Message>, rx: Receiver<Command>) -> JoinHandle<Result<(), Error>> {
        thread::spawn(|| Self::main(tx, rx))
    }

    fn main(tx: SyncSender<Message>, rx: Receiver<Command>) -> Result<(), Error> {
        let mut handler = Self::new(tx, rx)?;
        loop {
            match handler.rx.recv() {
                Ok(cmd) => handler.handle_cmd(cmd),
                Err(_) => continue,
            }

            let percentage = handler.playback_percentage();
            let seconds = handler.playback_pos_secs();

            match handler.tx.try_send(Message::Progress { percentage, seconds }) {
                Ok(_) => {},
                Err(std::sync::mpsc::TrySendError::Full(_)) => {},
                Err(std::sync::mpsc::TrySendError::Disconnected(_)) => panic!("audio message channel disconnected"),
            }
        }
    }

    fn handle_cmd(&mut self, cmd: Command) {
        match cmd {
            Command::Play(song) => self.play_song(song),
            Command::Pause => self.pause(),
            Command::Resume => self.resume(),
        }
    }

    fn play_song(&self, song: Song) {
        // TODO: error handling
        let path = crate::dirs().song_file(&song.ytid);
        let file = File::open(path).unwrap();
        let source = Decoder::new(file).unwrap();
        self.sink.clear();
        self.sink.append(source);
        self.resume();
    }

    fn pause(&self) {
        self.sink.pause();
    }

    fn resume(&self) {
        self.sink.play();
    }

    fn playback_pos_secs(&self) -> f32 {
        self.sink.get_pos().as_secs_f32().floor()
    }
    fn playback_remaining(&self) -> f32 {
        return self.duration() - self.playback_pos_secs();
    }
    fn duration(&self) -> f32 {
        match &self.current {
            Some(s) => Duration::from_secs(s.duration as u64).as_secs_f32(),
            None => 0.0
        }
    }
    fn playback_percentage(&self) -> f32 {
        let d = self.sink.get_pos();
        let t = Duration::from_secs(self.duration() as u64);
        if self.duration() == 0.0 { return 0.0 };
        let percentage = ((d.as_millis()/t.as_millis()) * 100) as f32;

        return percentage.clamp(0.0, 100.0);
    }
}


#[derive(Debug, Clone)]
enum Command {
    Play(Song),
    Pause,
    Resume,
}
#[derive(Debug, Clone)]
enum Message {
    Progress{ percentage: f32, seconds: f32 },
}

#[derive(Debug, Clone)]
enum LoopType {
    None,
    Song,
    Playlist
}
