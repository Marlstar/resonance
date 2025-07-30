use resonance::daemon::Daemon;
use iced::daemon::daemon;

fn main() -> iced::Result {
    resonance::db::setup();

    daemon(Daemon::boot, Daemon::update, Daemon::view)
        .title("Resonance")
        .subscription(Daemon::subscriptions)
        .title(Daemon::window_titles)
        .antialiasing(true)
        .run()
}

