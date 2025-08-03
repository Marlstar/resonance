use iced::futures::Stream;

pub fn subscription() -> impl Stream<Item = super::Endpoint> {
    super::endpoints::CHANNEL.1.clone()
}
