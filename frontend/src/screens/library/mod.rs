use iced::alignment::Horizontal;
use iced::widget::{ button, column, Column, scrollable };
use iced::{Background, Element};
use iced::Length;
use crate::screens::ScreenCore;
use crate::Task;
use crate::Message;
use backend::Song;

#[derive(Debug, Clone)]
pub struct Library {
    songs: Vec<Song>
}
impl Library {
    pub fn new(songs: Vec<Song>) -> Self {
        return Self {
            songs
        }
    }
}
impl ScreenCore for Library {
    type Message = LibraryMessage;

    fn view<'a>(&self, backend: &backend::Resonance) -> iced::Element<'a, crate::Message> {
        // let songs = self.songs.iter().map(|s| widgets::song(s, Message::PlaySong(s.clone()), true)).collect();
        let songs = self.songs.iter().map(|s| SONG_LINE_VIEW_BUILDER.build(s)).collect();
        let songs = Column::from_vec(songs)
            .width(Length::Fill)
            .spacing(10);

        let mut playing_button = button("Playing");
        if backend.audio.current_song.is_some() {
            playing_button = playing_button.on_press(Message::SwitchToPlayingScreen)
        }

        return Element::new(column![
            iced::widget::row![
                button("Home")
                .on_press(Message::SwitchToHomeScreen),

                playing_button,
            ],

            scrollable(songs)
                .width(Length::Fill),
        ]);
    }

    fn handle_message(&mut self, msg: Self::Message, backend: &mut backend::Resonance) -> Task {
        match msg {
            LibraryMessage::Refresh => self.refresh_songs(backend),
        }
    }
}
impl Library {
    fn refresh_songs(&mut self, backend: &mut backend::Resonance) -> Task {
        self.songs = backend.list_songs().unwrap();
        Task::none()
    }
}

#[derive(Debug, Clone)]
pub enum LibraryMessage {
    Refresh,
}


const SONG_LINE_VIEW_BUILDER: crate::widgets::song::line_view::Builder = crate::widgets::song::line_view::Builder {
    cover_click_message: |s| Message::PlaySong(s.clone()),
    background: Some(Background::Color(crate::appearance::colours::SURFACE0)),
    show_queue_button: true,
    alignment: Horizontal::Left,
    image_side: false,
};
