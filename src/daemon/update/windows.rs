use iced::window::Id;
use crate::daemon::Message;
use crate::iced::types::Task;
use crate::windows;

macro_rules! open_window {
    ($window:ident, $s:expr) => {{
        let (id, task) = windows::$window::open();
        $s.windows.$window = Some(id);
        notify_open(&stringify!($window), id);
        task.map(|_| Message::None)
    }}
}

impl super::super::Daemon {
    pub(super) fn open_main_window(&mut self) -> Task {
        open_window!(main, self)
    }

    pub(super) fn open_settings_window(&mut self) -> Task {
        open_window!(settings, self)
    }

    pub(super) fn handle_window_closed(&mut self, id: Id) -> Task {
        self.windows.update_closed(id);
        Task::none()
    }
}

fn notify_open(name: &str, id: Id) {
    println!("[window] opened {name} (id {id})")
}
