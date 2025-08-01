use iced::window::Id;
use crate::daemon::Message;
use crate::iced::types::Task;
use crate::windows;

impl super::super::Daemon {
    pub(super) fn open_main_window(&mut self) -> Task {
        let (id, task) = windows::main::open();
        self.windows.main = Some(id);
        notify_open("main", id);
        task.map(|_| Message::None)
    }

    pub(super) fn open_settings_window(&mut self) -> Task {
        let (id, task) = windows::settings::open();
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
