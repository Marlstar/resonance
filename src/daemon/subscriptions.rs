use iced::Subscription;
use crate::daemon::Message;

impl super::Daemon {
    pub fn subscriptions(&self) -> Subscription<Message> {
        Subscription::batch([
            Subscription::run(crate::tray::subscription).map(Message::Tray),
            iced::window::close_events().map(Message::WindowClosed),
        ])
    }
}
