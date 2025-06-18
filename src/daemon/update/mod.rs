use crate::daemon::Message;
use crate::iced::types::Task;

mod tray;

impl super::Daemon {
    pub fn update(&mut self, msg: Message) -> Task {
        match msg {
            // Dependencies
            Message::FFmpegDownloaded => { self.ffmpeg_ready = true; Task::none() },
            Message::YtDlpDownloaded => { self.ytdlp_ready = true; Task::none() },
        
            Message::Tray(event) => self.handle_tray_event(event),
        }
    }
}
