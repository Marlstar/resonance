use crate::iced::types::Element;
use crate::daemon::Daemon;
use crate::screens::Screen;
use iced::window::{Id, Settings};
use iced::Task;

pub fn open() -> (Id, Task<Id>) {
    iced::window::open(Settings {
        ..Default::default()
    })
}

pub fn view(daemon: &Daemon) -> Element {
    match daemon.current_screen {
        Screen::Library => daemon.screens.library.view(),
        Screen::Playing => daemon.screens.playing.fullscreen(daemon),
        _ => iced::widget::text("This screen shouldn't be focused on main!").into(),
    }
}
