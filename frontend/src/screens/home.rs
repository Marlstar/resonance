use iced::widget::{ button, column, text_input };
use iced::Element;
use crate::screens::ScreenCore;
use crate::Task;
use crate::Message;

#[derive(Debug, Clone)]
pub struct Home {
    download_url_input: String,
}
impl Home {
    pub fn new() -> Self {
        return Self {
            download_url_input: String::from(""),
        }
    }
}
impl Default for Home {
    fn default() -> Self {
        return Self::new()
    }
}
impl ScreenCore for Home {
    type Message = HomeMessage;

    fn view<'a>(&self) -> Element<'a, Message> {
        return Element::new(column![
            button("Download")
            .on_press(Message::Download(self.download_url_input.clone())),

            button("Delete song 1")
            .on_press(Message::DeleteSong(1)),

            text_input("Enter URL to download", &self.download_url_input)
            .on_input(|content| Message::Home(HomeMessage::DownloadURLChanged(content))),

            button("Library")
            .on_press(Message::SwitchToLibraryScreen)
        ])
    }

    fn handle_message(&mut self, message: HomeMessage) -> Task {
        match message {
            HomeMessage::DownloadURLChanged(content) => self.download_url_input = content,
        }

        Task::none()
    }
}

#[derive(Debug, Clone)]
pub enum HomeMessage {
    DownloadURLChanged(String),
}
