use std::io::Cursor;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use crate::models::Song;

pub struct AudioHandler {
    sink: Sink,
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,

    pub current: Option<Song>,
    pub volume: f32,
    pub position: i32,
    pub playing: bool,
}
impl AudioHandler {
    pub fn new() -> crate::Result<Self> {
        let (_stream, _stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&_stream_handle)?;

        return Ok(Self {
            sink, _stream, _stream_handle,
            current: None,
            volume: super::DEFAULT_VOLUME,
            position: 0,
            playing: false,
        });
    }
}
impl AudioHandler { // Actions
    pub fn load_song(&mut self, song: Song, bytes: Vec<u8>) -> crate::Result<()> {
        self.sink_load(bytes)?;
        self.current = Some(song);
        return Ok(());
    }
    fn sink_load(&mut self, bytes: Vec<u8>) -> crate::Result<()> {
        self.sink.clear();
        self.sink.append(Decoder::new(Cursor::new(bytes))?);
        return Ok(());
    }
    
    pub fn set_volume(&mut self, volume: f32) {
        self.sink.set_volume(volume);
        self.volume = volume;
    }

    pub fn resume(&mut self) {
        self.sink.play();
        self.playing = true;
    }

    pub fn pause(&mut self) {
        self.sink.pause();
        self.playing = false;
    }
}
impl AudioHandler { // Misc
    pub fn tick(&mut self) {
        self.position = self.sink_position();
    }

    fn sink_position(&self) -> i32 {
        return self.sink.get_pos().as_millis() as i32;
    }
}
