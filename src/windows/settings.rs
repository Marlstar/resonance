use crate::iced::types::Element;
use crate::daemon::Daemon;

#[derive(Default)]
pub struct SettingsWindow;
impl SettingsWindow {
    pub fn view(daemon: &Daemon) -> Element {
        daemon.screens.settings.view()
    }
}
