use iced::window::{Id, Settings};
use iced::Size;
use crate::{daemon::Message, iced::types::Task};

impl super::super::Daemon {
    pub(super) fn open_main_window(&mut self) -> Task {
        let (id, task) = iced::window::open(Settings::default());
        self.windows.main = Some(id);
        notify_open("main", id);
        task.map(|_| Message::None)
    }

    pub(super) fn open_settings_window(&mut self) -> Task {
        let settings = Settings {
            resizable: false,
            size: Size {width: 400.0, height: 600.0},
            ..Default::default()
        };
        let (id, task) = iced::window::open(settings);
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
