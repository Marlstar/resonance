use crate::iced::types::*;
use iced::widget;

impl super::Daemon {
    pub fn view(&self, window: iced::window::Id) -> Element {
        return widget::text("Hello, world!").into();
    }
}
