use crate::daemon::Message;
use crate::iced::types::Task;

mod tray;
mod ytdlp;
mod database;

impl super::Daemon {
    pub fn update(&mut self, msg: Message) -> Task {
        match msg {
            Message::None => Task::none(),

            // Dependencies
            Message::FFmpegDownloaded => { self.ffmpeg_ready = true; Task::none() },
            Message::YtDlpDownloaded => { self.ytdlp_ready = true; Task::none() },

            Message::GetSongMetadata(ytid) => self.get_song_metadata(ytid),
            Message::SongMetadata(job_id, result) => self.song_metadata_callback(job_id, result),

            Message::DownloadSong(song) => self.download_song(song),
            Message::SongDownload(song, result) => self.download_song_callback(song, result),

            // Database
            Message::InsertFailed(e) => self.handle_database_error(e),
        
            Message::Tray(event) => self.handle_tray_event(event),
        }
    }
}
