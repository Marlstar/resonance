use std::io::Cursor;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use crate::models::Song;

pub struct AudioHandler {
    sink: Sink,
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,

    current: Option<Song>,
    volume: f32,
    position: i32,
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
        });
    }

    pub fn load_next(&mut self, bytes: Vec<u8>) -> crate::Result<()> {
        self.sink.append(Decoder::new(Cursor::new(bytes))?);
        return Ok(());
    }
}
