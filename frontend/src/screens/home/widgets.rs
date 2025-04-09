use crate::appearance::styles;
use iced::widget::{container, text};
use iced::Element;

pub fn downloading<'a>(url: &str) -> Element<'a, crate::Message> {
    let song = text(backend::util::get_ytid_from_url(url).unwrap())
        .size(22);
    let container = container(song)
        .padding(3)
        .style(styles::rounded_bg_container);

    return container.into();
}
