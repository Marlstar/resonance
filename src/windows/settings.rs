use crate::iced::types::Element;
use crate::daemon::Daemon;
use iced::window::{Id, Settings};
use iced::Size;
use iced::Task;

pub fn open() -> (Id, Task<Id>) {
    iced::window::open(Settings {
        resizable: false,
        size: Size {width: 400.0, height: 600.0},
        ..Default::default()
    })
}

pub fn view(daemon: &Daemon) -> Element {
    daemon.screens.settings.view()
}
