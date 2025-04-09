use iced::widget::{ text, button, column, row, text_input, svg };
use iced::Element;
use crate::screens::ScreenCore;
use crate::Task;
use crate::Message;

mod widgets;

#[derive(Debug, Clone)]
pub struct Home {
    download_url_input: String,
    //downloading_songs: HashSet<String>,
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

    fn view<'a>(&self, backend: &backend::Resonance) -> Element<'a, Message> {
        let library_button = button("Library")
            .on_press(Message::SwitchToLibraryScreen);

        let download_button = button("Download")
            .on_press(Message::Download(self.download_url_input.clone()));
        let download_url_input = text_input("Enter URL to download", &self.download_url_input)
            .on_input(|content| Message::Home(HomeMessage::DownloadURLChanged(content)));

        let downloading = backend.downloading.iter().map(|url| widgets::downloading(url)).collect();

        return column![
            library_button,
            row![
                download_url_input,
                download_button,
            ],
            iced::widget::Column::from_vec(downloading),
            text(""),
            text(""),
            crate::assets::icon(),
        ].into();
    }

    fn handle_message(&mut self, message: HomeMessage, _backend: &mut backend::Resonance) -> Task {
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
