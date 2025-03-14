use resonance_frontend::Resonance;

fn main() -> iced::Result {
    iced::application("Resonance", Resonance::update, Resonance::view)
        .theme(Resonance::theme)
        .run()
}
