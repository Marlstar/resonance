use iced::Radians;
macro_rules! svg {
    ($n:ident, $f:expr) => {
        pub fn $n<'a>() -> iced::widget::Svg<'a> {
            const BYTES: &[u8] = include_bytes!($f);
            return iced::widget::svg(
                iced::advanced::svg::Handle::from_memory(BYTES)
            );
        }
    };
    ($n:ident, $f:expr, $rotation:expr) => {
        pub fn $n<'a>() -> iced::widget::Svg<'a> {
            const BYTES: &[u8] = include_bytes!($f);
            return iced::widget::svg(
                iced::advanced::svg::Handle::from_memory(BYTES)
            ).rotation($rotation);
        }
    }
}

svg!(icon, "resonance.svg");
svg!(pause, "pause.svg");
svg!(play, "play.svg");
svg!(skip_forward, "skip.svg");
svg!(skip_back, "skip.svg", Radians::PI);
svg!(queue_next, "queue_next.svg");
svg!(queue_end, "queue_end.svg");
svg!(fold_menu_right, "fold_menu.svg");
svg!(fold_menu_left, "fold_menu.svg", Radians::PI);

svg!(circle, "circle.svg");
