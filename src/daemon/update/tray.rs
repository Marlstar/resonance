use tray_icon::menu::MenuEvent;
use crate::iced::types::Task;
use crate::daemon::tasks;
use crate::daemon::Message;

impl super::super::Daemon {
    pub(super) fn handle_tray_event(&self, event: MenuEvent) -> Task {
        match event.id.0.as_str() {
            "open" => Task::done(Message::OpenMain),
            "exit" => tasks::exit(),
            other => { println!("[tray] event not handled: {other}"); Task::none() },
        }
    }
}
