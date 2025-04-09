macro_rules! svg {
    ($n:ident, $f:expr) => {
        pub fn $n() -> iced::advanced::svg::Handle {
            return iced::advanced::svg::Handle::from_memory(include_bytes!($f).as_slice());
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
