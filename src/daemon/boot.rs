use crate::iced::types::Task;
use crate::daemon::Message;
use crate::tasks;

impl super::Daemon {
    pub fn boot() -> (Self, Task) {
        let daemon = Self::new();
        let settings = &daemon.settings;

        let task = Task::batch([
            // Install dependencies
            tasks::install_deps::ffmpeg(),
            tasks::install_deps::ytdlp(),

            // Open main window (unless disabled)
            if !settings.start_minimised { Message::OpenMain.task() } else { Task::none() },

            Message::GetSongMetadata("2mXNRsyTitA".to_string()).task(),
        ]);
        return (daemon, task);
    }
}
