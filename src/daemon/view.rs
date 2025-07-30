use crate::iced::types::*;
use crate::windows::main::MainWindow;
use crate::windows::settings::SettingsWindow;

impl super::Daemon {
    pub fn view(&self, window: iced::window::Id) -> Element {
        if Some(window) == self.windows.main { MainWindow::view(self) }
        else if Some(window) == self.windows.settings { SettingsWindow::view(self) }
        else { iced::widget::text("Unknown window - make sure to render it in Daemon::view").into() }
    }
}
