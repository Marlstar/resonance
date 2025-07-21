use iced::window::{Id, Settings};
use crate::{daemon::Message, iced::types::Task};

impl super::super::Daemon {
    pub(super) fn open_main_window(&mut self) -> Task {
        let (id, task) = iced::window::open(Settings::default());
        self.windows.main = Some(id);
        task.map(|_| Message::None)
    }

    pub(super) fn handle_window_closed(&mut self, id: Id) -> Task {
        self.windows.update_closed(id);
        Task::none()
    }
}
