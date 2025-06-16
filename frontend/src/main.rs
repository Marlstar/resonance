use resonance_frontend::Resonance;

fn main() -> iced::Result {
    backend::deps::install_deps(); // Install ffmpeg and yt-dlp
    backend::db::setup::run_migrations(); // Setup database

    iced::application("Resonance", Resonance::update, Resonance::view)
        .theme(Resonance::theme)
        .subscription(Resonance::subscription)
        .window(Resonance::window_settings())
        .antialiasing(true)
        .run()
}
