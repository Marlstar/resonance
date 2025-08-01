use crate::daemon::Message;
use crate::iced::types::Task;

mod audio;
mod tray;
mod ytdlp;
mod database;
mod windows;
mod screens;
mod settings;

impl super::Daemon {
    pub fn update(&mut self, msg: Message) -> Task {
        match msg {
            Message::None => Task::none(),

            // Music control
            Message::LoadSong(song) => self.load_song(song),
            Message::LoadSongIntoSink(song, bytes) => self.load_song_into_sink(song, bytes),
            Message::Resume => { self.audio.resume(); Task::none() },
            Message::Pause => { self.audio.pause(); Task::none() },
            Message::Skip(offset) => todo!("skipping"),

            Message::OpenMain => self.open_main_window(),
            Message::OpenSettings => self.open_settings_window(),

            Message::FocusScreen(screen) => self.focus_screen(screen),

            // Dependencies
            Message::FFmpegDownloaded => { self.ffmpeg_ready = true; Task::none() },
            Message::YtDlpDownloaded => { self.ytdlp_ready = true; Task::none() },

            Message::SettingsUpdate(settings) => self.settings_update(settings),
            Message::SettingChanged(setting) => { self.screens.settings.handle_change(setting); Task::none() }

            Message::GetSongMetadata(ytid) => self.get_song_metadata(ytid),
            Message::SongMetadata(job_id, result) => self.song_metadata_callback(job_id, result),
            Message::SongInstalled(song) => self.handle_song_installed(song),

            Message::DownloadSong(song) => self.download_song(song),
            Message::SongDownload(result) => self.download_song_callback(result),

            // Database
            Message::DatabaseError(e) => self.handle_database_error(e),
        
            Message::Tray(event) => self.handle_tray_event(event),

            Message::WindowClosed(id) => self.handle_window_closed(id),
        }
    }
}
