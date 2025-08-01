use iced::futures::Stream;
use super::EVENT_RX;

pub fn subscription() -> impl Stream<Item = super::TrayEvent> {
    let rx = loop {
        if let Some(r) = EVENT_RX.get() { break r; }
    };
    rx.clone()
}
