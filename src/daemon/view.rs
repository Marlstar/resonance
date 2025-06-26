use crate::iced::types::*;
use crate::windows::main::MainWindow;

impl super::Daemon {
    pub fn view(&self, window: iced::window::Id) -> Element {
        if Some(window) == self.windows.main { MainWindow::view(self) }
        else { iced::widget::text("Unknown window").into() }
    }
}
