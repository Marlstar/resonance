use crate::daemon::Message;
use crate::iced::types::Task;

mod tray;
mod ytdlp;

impl super::Daemon {
    pub fn update(&mut self, msg: Message) -> Task {
        match msg {
            // Dependencies
            Message::FFmpegDownloaded => { self.ffmpeg_ready = true; Task::none() },
            Message::YtDlpDownloaded => { self.ytdlp_ready = true; Task::none() },

            Message::SongMetadata(job_id, result) => self.song_metadata_callback(job_id, result),
        
            Message::Tray(event) => self.handle_tray_event(event),
        }
    }
}
