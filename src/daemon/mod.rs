use crate::audio::handler::AudioHandler;
use crate::db::handler::DBHandler;
use crate::iced::types::Task;
use crate::tasks;

mod update;
mod view;
mod subscriptions;

mod message;
pub use message::Message;

pub struct Daemon {
    audio: AudioHandler,
    db: DBHandler,

    ffmpeg_ready: bool,
    ytdlp_ready: bool,
}
impl Daemon {
    pub fn new() -> Self {
        let audio = AudioHandler::new().expect("failed to initialise audio handler");
        let db = DBHandler::new().expect("failed to initialise DB handler");

        return Self {
            audio,
            db,
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
