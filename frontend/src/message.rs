use crate::screens;
use backend::SingleVideo;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Message {
    None,

    Mpris(backend::mpris::Recv),
    Seek(f32),
    SeekRelative(f32),
    SeekUpdate,

    /// Download a song by URL
    Download(String),
    DownloadComplete(String, SingleVideo),
    DownloadFailed(String, Arc<backend::Error>),

    DeleteSong(i32),

    PlaySong(backend::Song),
    Queue(backend::QueueEvent),
    Skip(isize),
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
