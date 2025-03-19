use iced::alignment::{Horizontal, Vertical};
use iced::widget::image::Handle;
use iced::widget::{ self, button, column, stack, text, container, svg };
use iced::{Background, Length};
use iced::Color;
use image::EncodableLayout;
use crate::screens::ScreenCore;
use crate::{appearance, Task};
use crate::Message;
use crate::assets;
use backend::Song;


#[derive(Debug, Clone)]
pub struct Playing {
    song: Song,
    playing: bool,
    bg: Box<[u8; 720*720*4]>,
}
impl Playing {
    pub fn new(song: Song) -> Self {
        return Self {
            bg: Box::new([0u8; 720*720*4]),
            song,
            playing: true,
        };
    }
}
impl ScreenCore for Playing {
    type Message = PlayingMessage;
    fn view<'a>(&self) -> iced::Element<'a, crate::Message> {
        let bg = widget::image(Handle::from_rgba(720, 720, self.bg.to_vec()))
        //let bg = widget::image(backend::dirs().song_thumbnail_blurred(&self.song.ytid))
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
        const BLUR_SIGMA: f32 = 75.0;

        self.song = song;
        let img = backend::blur(&self.song.ytid, BLUR_SIGMA);
        self.bg.copy_from_slice(&img);
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
