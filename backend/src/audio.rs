use std::fs::File;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TrySendError};
use std::time::Duration;
use crate::AM;
use orx_linked_list::{DoublyIdx, DoublyList, DoublyEnds, DoublyEndsMut, DoublyIterable};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
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
    pub queue: Queue,
    pub idx: DoublyIdx<Song>,
    pub current_song: Option<Song>,
    pub position: f32,
    pub progress: f32,
    pub loop_type: AM<LoopType>,

    // Frontend comms
    pub song_refresh_pending: bool,
}
impl AudioPlayer {
    pub fn new() -> Result<Self, Error> {
        let playing = false;
        let mut queue: Queue = DoublyList::new();
        let idx = queue.push_back(Song::NONE());
        let current_song: Option<Song> = None;

        let (ctx, crx) = sync_channel::<Command>(50);
        let (mtx, mrx) = sync_channel::<Message>(50);

        let loop_type = AM::new(LoopType::None);

        let _handle = AudioHandler::run(mtx, crx);

        let rt = tokio::runtime::Runtime::new().unwrap();

        Ok(Self {
            _handle,
            tx: ctx, rx: mrx,
            rt,
            playing, queue, idx, current_song,
            position: 0.0, progress: 0.0,
            loop_type,
            song_refresh_pending: false,
        })
    }

    fn handle_message(&mut self, message: Message) {
        // TODO: in future, don't send position updates as often, estimate it instead and update occationally?
        match message {
            Message::Progress { percentage, position: seconds } => {
                self.progress = percentage;
                self.position = seconds;
            },
            Message::Empty => {
                if self.song_queued_next() {
                    self.skip(true);
                    // TODO: nicer comms than this jank
                    self.song_refresh_pending = true;
                }
                else {
                    self.pause();
                }
            },
        }
    }

    pub fn current(&self) -> Option<Song> {
        return self.current_song.clone()
    }

    pub fn play_song(&mut self, song: Song) {
        self.load_song(song);
        self.resume();
    }

    pub fn load_song(&mut self, song: Song) {
        self.current_song = Some(song.clone());
        self.send_command(Command::Load(song));
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
        let emit = cmd.to_emit();
        self.rt.spawn(async move {
            let _ = crate::mpris::CTX.get().unwrap().send(emit).await;
        });

        // Rodio
        match self.tx.try_send(cmd) {
            Ok(_) => {},
            Err(TrySendError::Full(c)) => eprintln!("command channel full (sending {c:?})"),
            Err(TrySendError::Disconnected(_)) => panic!("audio command channel disconnected"),
        };
    }
    
    pub fn seek(&mut self, pos: f32) {
        self.send_command(Command::Seek(pos))
    }

    pub fn seek_relative(&mut self, offset: f32) {
        self.send_command(Command::Seek((self.position + offset).clamp(0.0, f32::MAX)))
    }

    pub fn seek_update(&mut self) {
        self.update();
    }

    fn update(&mut self) {
        if let Ok(m) = self.rx.try_recv() { self.handle_message(m); }
    }
}
impl AudioPlayer { // Queue
    /// Add a song to the end of the queue
    pub fn queue_add_back(&mut self, song: Song) {
        println!("[queue] adding '{}' to end of queue", &song.name);
        if let Some(s) = self.queue.get_mut(&self.idx) {
            if s.IS_NONE() {
                println!("  -> replacing dummy");
                *s = song;

                self.queue_post();
                return;
            }
        }
        self.queue.push_back(song);

        self.queue_post();
    }
    
    // /// Add a song to play next in the queue
    // pub fn queue_add_next(&mut self, song: Song) {
    //     let mut q = self.queue;
    // }

    #[allow(dead_code)] // TODO: remove
    fn queue_at_idx(&mut self, song: Song, idx: DoublyIdx<Song>, after: bool) {
        if after {
            self.queue.insert_next_to(&idx, song);
        } else {
            self.queue.insert_prev_to(&idx, song);
        }

        self.queue_post();
    }

    pub fn clear_queue(&mut self) {
        self.empty_queue_handler(None);
    }

    pub fn replace_queue(&mut self, song: Song) {
        self.empty_queue_handler(Some(song));
    }

    fn empty_queue_handler(&mut self, replacement: Option<Song>) {
        self.queue.clear();
        let song: Song;

        if let Some(r) = replacement {
            song = r;
            self.load_song(song.clone())
        } else {
            song = match &self.current_song {
                Some(s) => s.clone(),
                None => {
                    self.current_song = None;
                    Song::NONE()
                }
            };
        }

        self.idx = self.queue.push_front(song.clone());
        // TODO: no song (frontend compatibility) -> will panic if queue is cleared without replacement
        self.load_song(song);

        self.queue_post();
    }

    pub fn song_queued_next(&self) -> bool {
        self.queue.next_of(&self.idx).is_some()
    }

    fn queue_post(&mut self) {
        if self.current_song.is_none() {
            let song = self.queue.get(&self.idx).unwrap().clone();
            self.load_song(song);
        }
    }

    /// From current song backwards
    pub fn get_previous_songs(&self) -> Vec<Song> {
        use orx_linked_list::DoublyIterable;
        let mut before = self.queue.iter_backward_from(&self.idx)
            .map(Clone::clone)
            .skip(1) // ignore the current song
            .collect::<Vec<Song>>();
        before = before.into_iter().rev().collect::<Vec<Song>>();
        before
    }

    pub fn get_next_songs(&self) -> Vec<Song> {
        let after = self.queue.iter_from(&self.idx)
            .map(Clone::clone)
            .skip(1) // ignore the current song
            .collect::<Vec<Song>>();
        after
    }

    pub fn queue_with_offsets(&self) -> Vec<(isize, Song)> {
        let before = self.get_previous_songs().into_iter().enumerate().map(|(a,b)| (-(a as isize) - 1,b)).collect::<Vec<(isize, Song)>>();
        let current = self.current_song.as_ref().unwrap().clone(); // TODO: error handling for when there is no song playing at the start
        let after = self.get_next_songs().into_iter().enumerate().map(|(a,b)| ((a as isize)+1, b)).collect::<Vec<(isize, Song)>>();

        let mut songs = before;
        songs.push((0,current));
        for s in after {
            songs.push(s)
        }

        songs
    }

    // Queue interaction
    // TODO: batch skip
    pub fn skip(&mut self, forward: bool) -> bool {
        // TODO: remove
        let next = if forward {
            self.queue.next_idx_of(&self.idx)
        } else {
            self.queue.prev_idx_of(&self.idx)
        };
        if let Some(next) = next {
            let s = self.queue.get(&next).unwrap().clone();
            self.current_song = Some(s.clone());
            self.idx = next;
            self.play_song(s);
            return true;
        }
        else {
            println!("No song in queue to skip to!");
        }
        return false;
    }
}

#[allow(dead_code)]
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
        let mpris_tx = crate::mpris::CTX.get().unwrap();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let mut pos = 0f32;
        let mut was_empty = false;
        loop {
            // Handle pending commands
            while let Ok(cmd) = handler.rx.try_recv() {
                handler.handle_cmd(cmd)
            }

            // Song completion
            if handler.sink.empty() {
                if !was_empty {
                    handler.tx.try_send(Message::Empty).unwrap();
                    was_empty = true;
                }
            } else { was_empty = false; }

            // Progress updates
            let percentage = handler.playback_percentage();
            let new = handler.playback_pos();
            if (new - pos).abs() > 0.2 {
                match handler.tx.try_send(Message::Progress { percentage, position: new }) {
                    Ok(_) => {
                        rt.spawn(async move { mpris_tx.send(Emit::Seek(new)).await });
                    },
                    Err(std::sync::mpsc::TrySendError::Full(_)) => {},
                    Err(std::sync::mpsc::TrySendError::Disconnected(_)) => panic!("audio message channel disconnected"),
                }
                pos = new;
            }
        }
    }

    fn handle_cmd(&mut self, cmd: Command) {
        match cmd {
            Command::Load(song) => self.load_song(song),
            Command::Pause => self.pause(),
            Command::Resume => self.resume(),
            Command::Seek(pos) => self.seek(pos),
        }
    }

    #[allow(dead_code)]
    fn play_song(&self, song: Song) {
        self.load_song(song);
        self.resume();
    }
    
    fn load_song(&self, song: Song) {
        // TODO: error handling
        let path = crate::dirs().song_file(&song.ytid);
        let file = File::open(path).unwrap();
        let source = Decoder::new(file).unwrap();
        self.sink.clear();
        self.sink.append(source);
    }

    fn pause(&self) {
        self.sink.pause();
    }

    fn resume(&self) {
        self.sink.play();
    }

    // FIX: seeking does not work if song has finished
    fn seek(&self, pos: f32) {
        self.sink.try_seek(Duration::from_secs_f32(pos)).unwrap();
    }

    pub fn playback_pos(&self) -> f32 {
        self.sink.get_pos().as_secs_f32()
    }

    #[allow(dead_code)]
    pub fn playback_remaining(&self) -> f32 {
        return self.duration() - self.playback_pos();
    }
    pub fn duration(&self) -> f32 {
        if let Some(s) = &self.current {
            s.duration as f32
        } else {
            0.0
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
    Load(Song),
    Pause,
    Resume,
    Seek(f32),
}
impl From<Command> for Emit {
    fn from(value: Command) -> Self {
        Command::to_emit(&value)
    }
}
impl Command {
    pub fn to_emit(&self) -> Emit {
        match self {
            Command::Pause => Emit::Pause,
            Command::Resume => Emit::Play,
            Command::Load(s) => Emit::Song(s.clone()),
            Command::Seek(pos) => Emit::Seek(*pos),
        }
    }
}
#[derive(Debug, Clone)]
enum Message {
    Progress { percentage: f32, position: f32 },
    Empty,
}

#[derive(Debug, Clone)]
pub enum QueueEvent {
    AddToEnd(Song),
    AddNext(Song),
}

#[derive(Debug, Clone)]
pub enum LoopType {
    None,
    Song,
    Playlist
}
