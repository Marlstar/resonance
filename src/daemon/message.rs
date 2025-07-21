use std::sync::Arc;
use crate::models::Song;

#[derive(Debug, Clone)]
pub enum Message {
    None,

    // Windows
    OpenMain,

    FFmpegDownloaded,
    YtDlpDownloaded,

    GetSongMetadata(String),
    SongMetadata(String, Arc<crate::Result<Box<youtube_dl::SingleVideo>>>),

    DownloadSong(Song),
    SongDownload(Arc<crate::Result<Song>>),

    // Database
    DatabaseError(Arc<diesel::result::Error>),

    Tray(tray_icon::menu::MenuEvent),

    WindowClosed(iced::window::Id),
}
