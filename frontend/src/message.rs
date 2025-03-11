use crate::screens::{self, Screen};
use crate::backend::SingleVideo;

#[derive(Debug, Clone)]
pub enum Message {
    SwitchScreen(Screen),

    /// Download a song by URL
    Download(String),
    DownloadComplete(String, SingleVideo),
    DownloadFailed(String),

    DeleteSong(i32),


    // Screens
    Home(screens::home::HomeMessage),
}
