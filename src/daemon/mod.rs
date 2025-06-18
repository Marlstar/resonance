use crate::audio::handler::AudioHandler;
use crate::iced::types::Task;

mod update;
mod view;
mod subscriptions;

pub mod tasks;

mod message;
pub use message::Message;

pub struct Daemon {
    audio: AudioHandler,

    ffmpeg_ready: bool,
    ytdlp_ready: bool,
}
impl Daemon {
    pub fn new() -> Self {
        let audio = AudioHandler::new().expect("failed to initialise audio handler");

        return Self {
            audio,
            ffmpeg_ready: false,
            ytdlp_ready: false,
        };
    }

    pub fn boot() -> (Self, Task) {
        let task = Task::batch([
            tasks::install_deps::ffmpeg(),
            tasks::install_deps::ytdlp(),
        ]);
        return (Self::new(), task);
    }
}
impl Default for Daemon {
    fn default() -> Self { Self::new() }
}
