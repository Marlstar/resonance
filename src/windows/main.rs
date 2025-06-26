use crate::iced::types::Element;
use crate::daemon::Daemon;

#[derive(Default)]
pub struct MainWindow;
impl MainWindow {
    pub fn view(daemon: &Daemon) -> Element {
        daemon.screens.playing.fullscreen(daemon)
    }
}
