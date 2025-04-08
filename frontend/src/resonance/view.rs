use iced::Element;
use crate::Message;

impl super::Resonance {
    pub fn view(&self) -> Element<Message> {
        self.screen.view(&self.backend)
    }
}
