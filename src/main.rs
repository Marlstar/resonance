use resonance::daemon::Daemon;
use iced::daemon::daemon;

fn main() -> iced::Result {
    resonance::db::setup();

    let _tray = resonance::tray::TrayHandler::new();

    daemon(Daemon::boot, Daemon::update, Daemon::view)
        .title("Resonance")
        .subscription(Daemon::subscriptions)
        .antialiasing(true)
        .run()
}

