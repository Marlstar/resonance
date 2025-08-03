use crate::iced::types::Task;
use crate::tasks;

impl super::Daemon {
    pub fn exit(&mut self) -> Task {
        println!("[main] shutting down");
        Task::batch([
            tasks::settings::save_to_file()
        ]).chain(crate::tasks::exit())
    }
}
