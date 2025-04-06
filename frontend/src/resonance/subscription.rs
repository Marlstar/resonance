use std::time::Duration;
use iced::task::{sipper, Never};
use iced::{task::Sipper, Subscription};
use crate::Message;

impl super::Resonance {
    pub fn subscription(&self) -> Subscription<Message> {
        mpris()
    }
}

fn mpris() -> Subscription<Message> {
    //Subscription::run(mpris_stream)
    Subscription::batch([
        Subscription::run(mpris_stream),
        mpris_send_seek(),
    ])
}
fn mpris_stream() -> impl Sipper<Never, Message> {
    sipper(async |mut output| {
        let mut rx = backend::mpris::ERX.get().unwrap().lock().await;
        loop {
            let recv = rx.recv().await.unwrap();
            output.send(Message::Mpris(recv)).await;
        }
    })
}
fn mpris_send_seek() -> Subscription<Message> {
    iced::time::every(Duration::from_millis(250)).map(|_| Message::SeekUpdate)
}
