pub fn icon_png() -> iced::advanced::image::Handle {
    return iced::advanced::image::Handle::from_bytes(include_bytes!("resonance.png").as_slice())
}
macro_rules! svg {
    ($n:ident, $f:expr) => {
        pub fn $n() -> iced::advanced::svg::Handle {
            return iced::advanced::svg::Handle::from_memory(include_bytes!($f).as_slice());
        }
    }
}
//pub fn icon() -> iced::advanced::svg::Handle {
//    return iced::advanced::svg::Handle::from_memory(include_bytes!("resonance.svg").as_slice())
//}
svg!(icon, "resonance.svg");
svg!(pause, "pause.svg");
svg!(play, "play.svg");

svg!(circle, "circle.svg");
