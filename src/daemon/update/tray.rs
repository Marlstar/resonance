use tray_icon::menu::MenuEvent;
use crate::iced::types::Task;
use crate::daemon::tasks;
use crate::daemon::Message;

impl super::super::Daemon {
    pub(super) fn handle_tray_event(&mut self, event: MenuEvent) -> Task {
        match event.id.0.as_str() {
            "open" => Message::OpenMain.task(),
            "settings" => Message::OpenSettings.task(),
            "exit" => { self.exit(); tasks::exit() },
            other => { println!("[tray] event not handled: {other}"); Task::none() },
        }
    }
}
