use iced::widget::container;
use iced::{Background, Border, Theme};
use crate::appearance::colours;

pub fn rounded_bg_container(_: &Theme) -> container::Style {
    container::Style::default()
        .background(Background::Color(colours::SURFACE0))
        .border(Border::default().rounded(10))
}
