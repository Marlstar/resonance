use crate::iced::types::Element;
use crate::daemon::Daemon;
use iced::window::{Id, Settings};
use iced::Task;

pub fn open() -> (Id, Task<Id>) {
    iced::window::open(Settings {
        ..Default::default()
    })
}

pub fn view(daemon: &Daemon) -> Element {
    daemon.screens.playing.fullscreen(daemon)
}
