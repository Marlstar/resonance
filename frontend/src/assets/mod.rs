pub fn icon() -> iced::advanced::image::Handle {
    return iced::advanced::image::Handle::from_bytes(include_bytes!("resonance.png").as_slice())
}
pub fn icon_svg() -> iced::advanced::svg::Handle {
    return iced::advanced::svg::Handle::from_memory(include_bytes!("resonance.svg").as_slice())
}
