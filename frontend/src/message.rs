use crate::screens;
use backend::SingleVideo;

#[derive(Debug, Clone)]
pub enum Message {
    Mpris(backend::mpris::Recv),

    /// Download a song by URL
    Download(String),
    DownloadComplete(String, SingleVideo),
    DownloadFailed(String),

    DeleteSong(i32),

    PlaySong(i32),
    ResumeSong,
    PauseSong,

    // Screens
    Home(screens::HomeMessage),
    SwitchToHomeScreen,

    Library(screens::LibraryMessage),
    SwitchToLibraryScreen,


    Playing(screens::PlayingMessage),
    SwitchToPlayingScreen,
}
