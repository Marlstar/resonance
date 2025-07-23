use std::sync::Arc;
use crate::models::Song;

#[derive(Debug, Clone)]
pub enum Message {
    None,
    
    // Music control
    LoadSong(Song),
    LoadSongIntoSink(Song, Vec<u8>),
    Resume,
    Pause,
    Skip(isize),

    // Windows
    OpenMain,

    FFmpegDownloaded,
    YtDlpDownloaded,

    GetSongMetadata(String),
    SongMetadata(String, Arc<crate::Result<Box<youtube_dl::SingleVideo>>>),
    SongInstalled(Song),

    DownloadSong(Song),
    SongDownload(Arc<crate::Result<Song>>),

    // Database
    DatabaseError(Arc<diesel::result::Error>),

    Tray(tray_icon::menu::MenuEvent),

    WindowClosed(iced::window::Id),
}

impl Message {
    pub fn task(self) -> iced::Task<Message> {
        iced::Task::<Message>::done(self)
    }
}
impl From<Message> for iced::Task<Message> {
    fn from(value: Message) -> Self {
        value.task()
    }
}
