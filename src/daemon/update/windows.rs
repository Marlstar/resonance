use iced::window::Id;
use crate::{daemon::Message, iced::types::Task};

impl super::super::Daemon {
    pub(super) fn open_main_window(&mut self) -> Task {
        let (id, task) = crate::windows::main::MainWindow::open();
        self.windows.main = Some(id);
        notify_open("main", id);
        task.map(|_| Message::None)
    }

    pub(super) fn open_settings_window(&mut self) -> Task {
        let (id, task) = crate::windows::settings::SettingsWindow::open();
        self.windows.settings = Some(id);
        notify_open("settings", id);
        task.map(|_| Message::None)
    }

    pub(super) fn handle_window_closed(&mut self, id: Id) -> Task {
        self.windows.update_closed(id);
        Task::none()
    }
}

fn notify_open(name: &str, id: Id) {
    println!("[window] opened {name} (id {id})")
}
