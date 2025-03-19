use iced::alignment::{Horizontal, Vertical};
use iced::widget::{ self, button, column, stack, text, container, svg };
use iced::{Background, Length};
use iced::Color;
use crate::screens::ScreenCore;
use crate::{appearance, Task};
use crate::Message;
use crate::assets;
use backend::Song;


#[derive(Debug, Clone)]
pub struct Playing {
    song: Song,
    playing: bool,
}
impl Playing {
    pub fn new(song: Song) -> Self {
        return Self {
            song,
            playing: true,
        };
    }
}
impl ScreenCore for Playing {
    type Message = PlayingMessage;
    fn view<'a>(&self) -> iced::Element<'a, crate::Message> {
        let bg = widget::image(backend::dirs().song_thumbnail_blurred(&self.song.ytid))
            .width(Length::Fill)
            .height(Length::Fill)
            .content_fit(iced::ContentFit::Cover);

        let icon = if self.playing {assets::pause()} else {assets::play()};
        let icon = svg(icon);
            //.style(appearance::styles::colour_svg(Color::BLACK));
        let pause_play = button(container(icon))
            .width(Length::Fixed(80.0))
            .on_press(if self.playing {Message::PauseSong} else {Message::ResumeSong})
            .style(|_,_| button::Style::default());
        //let pause_play = button("hello");


        stack!(
            bg,
            column![
                button("Library")
                .on_press(Message::SwitchToLibraryScreen),

                text(self.song.name.clone()),

                container(pause_play)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center)
                    .width(Length::Fill)
                    .height(Length::Fill)
            ]
        ).into()
    }

    fn handle_message(&mut self, msg: Self::Message) -> crate::Task {
        match msg {
            PlayingMessage::Update(s) => self.update_song(s),
            PlayingMessage::PlayingStatus(playing) => self.update_playing(playing),
        }
    }
}
impl Playing {
    fn update_song(&mut self, song: Song) -> Task {
        self.song = song;
        // TODO: check if blurred image is saved to file, and if not, create it
        Task::none()
    }

    fn update_playing(&mut self, p: bool) -> Task {
        self.playing = p;
        return Task::none()
    }
}

#[derive(Debug, Clone)]
pub enum PlayingMessage {
    Update(Song), // When song changes
    PlayingStatus(bool),
}
