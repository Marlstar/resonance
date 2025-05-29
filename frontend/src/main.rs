use resonance_frontend::Resonance;

fn main() -> iced::Result {
    backend::deps::install_deps();

    iced::application("Resonance", Resonance::update, Resonance::view)
        .theme(Resonance::theme)
        .subscription(Resonance::subscription)
        .window(Resonance::window_settings())
        .run()
}
