use crate::audio::handler::AudioHandler;
use crate::windows::Windows;
use crate::screens::Screens;
use crate::tasks;

mod update;
mod view;
mod subscriptions;
mod boot;

mod message;
pub use message::Message;

pub struct Daemon {
    pub audio: AudioHandler,
    
    pub windows: Windows,
    pub screens: Screens,

    pub ffmpeg_ready: bool,
    pub ytdlp_ready: bool,
}
impl Daemon {
    pub fn new() -> Self {
        let audio = AudioHandler::new().expect("failed to initialise audio handler");

        return Self {
            audio,
            windows: Windows::default(),
            screens: Screens::default(),
            ffmpeg_ready: false,
            ytdlp_ready: false,
        };
    }
}
impl Default for Daemon {
    fn default() -> Self { Self::new() }
}
