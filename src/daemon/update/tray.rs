use crate::iced::types::Task;
use crate::daemon::tasks;
use crate::daemon::Message;
use crate::tray::TrayEvent;

impl super::super::Daemon {
    pub(super) fn handle_tray_event(&mut self, event: TrayEvent) -> Task {
        match event {
            TrayEvent::Open => Message::OpenMain.task(),
            TrayEvent::Settings => Message::OpenSettings.task(),
            TrayEvent::Exit => { self.exit(); tasks::exit() },
        }
    }
}
