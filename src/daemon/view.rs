use crate::iced::types::*;
use crate::windows;

impl super::Daemon {
    pub fn view(&self, window: iced::window::Id) -> Element {
        if Some(window) == self.windows.main { windows::main::view(self) }
        else if Some(window) == self.windows.settings { windows::settings::view(self) }
        else { iced::widget::text("Unknown window - make sure to render it in Daemon::view").into() }
    }
}
