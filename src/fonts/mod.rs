use iced::font::{Font, Weight};
use std::sync::LazyLock;

pub static BOLD: LazyLock<Font> = LazyLock::new(|| Font {
    weight: Weight::Bold,
    ..Default::default()
});
