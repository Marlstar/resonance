use iced::alignment::{Horizontal, Vertical};
use iced::widget::image::Handle;
use iced::widget::{ self, button, column, container, row, stack, svg, text, Space };
use iced::{Font, Length};
use crate::screens::ScreenCore;
use crate::Task;
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
            .width(Length::Fill)
            .height(Length::Fill)
            .content_fit(iced::ContentFit::Cover);

        let icon = if self.playing {assets::pause()} else {assets::play()};
        let icon = svg(icon);
            //.style(appearance::styles::colour_svg(Color::BLACK));
        let pause_play = button(container(icon))
            .width(Length::Fixed(83.0))
            .on_press(if self.playing {Message::PauseSong} else {Message::ResumeSong})
            .style(|_,_| button::Style::default());
        let pause_play = container(pause_play)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center);

        let bold_font = Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        };

        let title = text(self.song.name.clone())
            .size(22.0)
            .font(bold_font);
        let author = text(self.song.author.clone())
            .size(16);
        let album = text(self.song.album.clone())
            .size(16);

        let song_info = column![
            title,
            row![author, text("·"), album].spacing(5),
        ].align_x(Horizontal::Center).spacing(0.0);

        let info = column![
            song_info,
            pause_play,
        ].align_x(Horizontal::Center).spacing(30.0);

        let cover = iced::widget::image(backend::dirs().song_thumbnail(&self.song.ytid))
            .width(300)
            .height(300);

        let main = row![
            cover,
            Space::with_width(Length::Fixed(50.0)),
            info,
            Space::with_width(Length::Fixed(30.0)),
        ].spacing(0).align_y(Vertical::Center).width(Length::Shrink);

        stack!(
            bg,
            column![
                button("Library")
                .on_press(Message::SwitchToLibraryScreen),

                container(main)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center),

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
