use iced::widget::{text, svg, container} ;
use iced::{Background, Border, Theme};
use crate::appearance::colours;

pub fn grey_text(_: &Theme) -> text::Style {
    text::Style {
        color: Some(colours::OVERLAY2)
    }
}

pub fn rounded_bg_container(_: &Theme) -> container::Style {
    container::Style::default()
        .background(Background::Color(colours::SURFACE0))
        .border(Border::default().rounded(10))
}

pub fn colour_svg(color: iced::Color) -> impl Fn(&Theme, svg::Status) -> svg::Style {
    move |_: &Theme, _: svg::Status| svg::Style {
        color: Some(color)
    }
}
