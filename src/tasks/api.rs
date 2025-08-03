use crate::iced::types::Task;
use crate::daemon::Message;

pub fn run() -> Task {
    iced::Task::future(crate::api::run())
        .map(|_| {
            println!("API shutdown! This shouldn't happen.");
            Message::None
        })
}

