use crate::Message;
use backend::Song;
use crate::appearance::styles;
use iced::alignment::Vertical;
use iced::widget::{ button, column, container, hover, image, svg, row, text };
use iced::{Background, Border, Element, Length, Theme};

pub fn downloading<'a>(url: &str) -> Element<'a, crate::Message> {
    let song = text(url.to_string())
        .size(22);
    let container = container(song)
        .style(styles::rounded_bg_container);

    return container.into();
}
