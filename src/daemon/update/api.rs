use crate::daemon::Message;
use crate::iced::types::Task;
use crate::api::prelude::*;

impl super::super::Daemon {
    pub(super) fn handle_api(&self, endpoint: Endpoint) -> Task {
        println!("[api] {endpoint:?}");
        match endpoint {
            Endpoint::Control(action) => self.control(action),
        }
    }

    fn control(&self, action: Control) -> Task {
        match action {
            Control::Resume => Message::Resume.task(),
            Control::Pause => Message::Pause.task(),
        }
    }
}
