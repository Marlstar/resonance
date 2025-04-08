use backend::util::format_duration;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::image::Handle;
use iced::widget::{ self, button, column, container, row, slider, stack, svg, text, Space };
use iced::{Background, Color, Font, Length};
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
    pos: f32,
}
impl Playing {
    pub fn new(song: Song) -> Self {
        return Self {
            bg: Box::new([0u8; 720*720*4]),
            song,
            playing: true,
            pos: 0f32,
        };
    }
}
impl ScreenCore for Playing {
    type Message = PlayingMessage;
    fn view<'a>(&self, backend: &backend::Resonance) -> iced::Element<'a, crate::Message> {
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

        let skip_forward = button(container(svg(assets::skip_forward())))
            .width(Length::Fixed(83.0))
            .on_press(Message::Skip(1))
            .style(|_,_| button::Style::default());
        let skip_forward = container(skip_forward)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center);

        let skip_back = button(container(svg(assets::skip_back())))
            .width(Length::Fixed(83.0))
            .on_press(Message::Skip(-1))
            .style(|_,_| button::Style::default());
        let skip_back = container(skip_back)
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

        let slider = slider(0.0..=(self.song.duration as f32), self.pos, Message::Seek)
            .width(Length::Fill);

        let pos = text(format_duration(self.pos as usize));
        let duration = text(format_duration(self.song.duration as usize));
        // let time = row![pos, Space::new(Length::Fill, Length::Fixed(0.0)), duration]
        let time = row![pos, slider, duration]
            .spacing(10.0)
            .align_y(Vertical::Center);

        let song_info = column![
            title,
            row![author, text("·"), album].spacing(5),
        ].align_x(Horizontal::Center).spacing(0.0);

        let controls = row![skip_back, pause_play, skip_forward]
            .align_y(Vertical::Center);

        let info = column![
            song_info,
            controls,
            time,
            Space::new(Length::Fixed(400.0), Length::Fixed(0.0)),
        ].align_x(Horizontal::Center).width(Length::Shrink).spacing(30.0);

        let cover = iced::widget::image(backend::dirs().song_thumbnail(&self.song.ytid))
            .width(300)
            .height(300);

        let main = row![
            cover,
            Space::with_width(Length::Fixed(50.0)),
            info,
            Space::with_width(Length::Fixed(30.0)),
        ].spacing(0).align_y(Vertical::Center);//.width(Length::Shrink);

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
            PlayingMessage::Seek(pos) => self.seeked(pos),
        }
    }
}
impl Playing {
    fn update_song(&mut self, song: Song) -> Task {
        const BLUR_SIGMA: f32 = 75.0;

        self.pos = 0.0;
        self.song = song;
        let img = backend::blur(&self.song.ytid, BLUR_SIGMA);
        self.bg.copy_from_slice(&img);
        Task::none()
    }

    fn update_playing(&mut self, p: bool) -> Task {
        self.playing = p;
        return Task::none()
    }

    fn seeked(&mut self, pos: f32) -> Task {
        self.pos = pos;
        Task::none()
    }
}

#[derive(Debug, Clone)]
pub enum PlayingMessage {
    Update(Song), // When song changes
    PlayingStatus(bool),
    Seek(f32),
}
