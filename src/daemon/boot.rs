use crate::iced::types::Task;
use crate::daemon::Message;
use crate::tasks;

impl super::Daemon {
    pub fn boot() -> (Self, Task) {
        let task = Task::batch([
            tasks::install_deps::ffmpeg(),
            tasks::install_deps::ytdlp(),
            Message::OpenMain.task(),
        ]);
        return (Self::new(), task);
    }
}
