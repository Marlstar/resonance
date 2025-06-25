use std::sync::Arc;
use crate::models::Song;

#[derive(Debug, Clone)]
pub enum Message {
    None,

    FFmpegDownloaded,
    YtDlpDownloaded,

    GetSongMetadata(String),
    SongMetadata(String, Arc<crate::Result<Box<youtube_dl::SingleVideo>>>),

    DownloadSong(Song),
    SongDownload(Song, Arc<crate::Result<()>>),

    // Database
    InsertFailed(Arc<diesel::result::Error>),

    Tray(tray_icon::menu::MenuEvent),
}
