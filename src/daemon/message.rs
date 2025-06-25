use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Message {
    None,

    FFmpegDownloaded,
    YtDlpDownloaded,

    GetSongMetadata(String),
    SongMetadata(String, Arc<crate::Result<Box<youtube_dl::SingleVideo>>>),

    // Database
    InsertFailed(Arc<diesel::result::Error>),

    Tray(tray_icon::menu::MenuEvent),
}
