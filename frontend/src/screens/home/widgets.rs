use crate::appearance::styles;
use iced::widget::{container, text};
use iced::Element;

pub fn downloading<'a>(url: &str) -> Element<'a, crate::Message> {
    let song = text(url.to_string())
        .size(22);
    let container = container(song)
        .style(styles::rounded_bg_container);

    return container.into();
}
