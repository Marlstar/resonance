macro_rules! svg {
    ($n:ident, $f:expr) => {
        pub fn $n<'a>() -> iced::widget::Svg<'a> {
            const BYTES: &[u8] = include_bytes!($f);
            return iced::widget::svg(
                iced::advanced::svg::Handle::from_memory(BYTES)
            );
        }
    }
}
svg!(icon, "resonance.svg");
svg!(pause, "pause.svg");
svg!(play, "play.svg");
svg!(skip_forward, "skip_forward.svg");
svg!(skip_back, "skip_back.svg");
svg!(queue_next, "queue_next.svg");
svg!(queue_end, "queue_end.svg");

svg!(circle, "circle.svg");
