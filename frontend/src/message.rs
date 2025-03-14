use crate::screens;
use crate::backend::SingleVideo;

#[derive(Debug, Clone)]
pub enum Message {
    /// Download a song by URL
    Download(String),
    DownloadComplete(String, SingleVideo),
    DownloadFailed(String),

    DeleteSong(i32),

    PlaySong(i32),

    // Screens
    Home(screens::HomeMessage),
    SwitchToHomeScreen,

    Library(screens::LibraryMessage),
    SwitchToLibraryScreen,
}
