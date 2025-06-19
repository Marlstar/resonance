use crate::iced::types::Task;
use crate::daemon::Message;

pub fn ffmpeg() -> Task {
    super::run_and_return_message!(crate::deps::ffmpeg::install, Message::FFmpegDownloaded);
}

pub fn ytdlp() -> Task {
    super::run_and_return_message!(crate::deps::ytdlp::install, Message::YtDlpDownloaded);
}
