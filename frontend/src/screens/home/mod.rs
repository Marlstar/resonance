use hashbrown::HashSet;
use iced::widget::{ button, column, row, text_input, Column };
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
            //downloading_songs: HashSet::new()
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
        let library_button = button("Library")
            .on_press(Message::SwitchToLibraryScreen);

        let download_button = button("Download")
            .on_press(Message::Download(self.download_url_input.clone()));
        let download_url_input = text_input("Enter URL to download", &self.download_url_input)
            .on_input(|content| Message::Home(HomeMessage::DownloadURLChanged(content)));

        //let downloading = self.downloading_songs.iter().map(|url| widgets::downloading(url)).collect();

        return column![
            library_button,
            row![
                download_url_input,
                download_button,
            ],
            //Column::from_vec(downloading)
        ].into();
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
