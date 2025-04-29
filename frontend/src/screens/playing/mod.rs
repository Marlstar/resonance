use backend::linked_list::DoublyEnds;
use backend::util::format_duration;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::image::Handle;
use iced::widget::{ self, button, column, container, row, scrollable, slider, stack, text, Column, Space };
use iced::Length::Fill;
use iced::{Font, Length};
use crate::screens::ScreenCore;
use crate::Task;
use crate::Message;
use crate::assets;
use backend::Song;

#[derive(Debug, Clone)]
pub struct Playing {
    song: Song,
    bg: Box<[u8; 720*720*4]>,
    pos: f32,
    queue_shown: bool,
}
impl Playing {
    pub fn new(song: Song) -> Self {
        return Self {
            bg: Box::new([0u8; 720*720*4]),
            song,
            pos: 0f32,
            queue_shown: true,
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

        let icon = if backend.audio.playing {assets::pause()} else {assets::play()};
            //.style(appearance::styles::colour_svg(Color::BLACK));
        let pause_play = button(container(icon))
            .width(Length::Fixed(83.0))
            .on_press(if backend.audio.playing {Message::PauseSong} else {Message::ResumeSong})
            .style(|_,_| button::Style::default());
        let pause_play = container(pause_play)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center);

        let skip_forward = button(container(assets::skip_forward()))
            .width(Length::Fixed(83.0))
            .on_press(Message::Skip(1))
            .style(|_,_| button::Style::default());
        let skip_forward = container(skip_forward)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center);

        let skip_back = button(container(assets::skip_back()))
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
        let main = container(main)
            .align_x(Horizontal::Center);

        let mut main = row![
            main.width(Length::FillPortion(2)),
        ].align_y(Vertical::Center);

        if self.queue_shown {
            let queue = self.queue(backend);
            let queue = container(queue)
                .align_x(Horizontal::Center);
            main = main.push(queue);
        }

        // ====
        let library_button = button("Library")
            .on_press(Message::SwitchToLibraryScreen);
        
        let queue_flyout_button = button(if self.queue_shown {assets::fold_menu_right()} else {assets::fold_menu_left()})
            .style(|_,_| button::Style::default())
            .width(40.0)
            .on_press(Message::Playing(PlayingMessage::QueueShown(!self.queue_shown)));

        let topbar = row![library_button, Space::new(Fill, 0.0), queue_flyout_button]
            .align_y(Vertical::Top);

        stack!(
            bg,
            column![
                topbar,

                container(main)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center),

            ]
        ).into()
    }

    fn handle_message(&mut self, msg: Self::Message, backend: &mut backend::Resonance) -> crate::Task {
        let t1 = self.check_backend_updates(backend, &msg);
        let t2 = match msg {
            PlayingMessage::SongUpdate => self.update_song(backend.audio.current_song.clone()),
            PlayingMessage::PlaybackUpdate => Task::none(),
            PlayingMessage::PositionUpdate => self.position_update(backend.audio.position),
            PlayingMessage::QueueShown(shown) => {
                self.queue_shown = shown;
                Task::none()
            },
        };

        Task::batch([t1, t2])
    }
}
impl Playing {
    fn queue<'a>(&self, backend: &backend::Resonance) -> iced::Element<'a, Message> {
        // TODO: move some of this logic to the backend
        use backend::linked_list::DoublyIterable;
        let mut before = backend.audio.queue.iter_backward_from(&backend.audio.idx)
            .enumerate()
            .map(|(a,b)| (-(a as isize), b.clone()))
            .collect::<Vec<(isize, Song)>>();
        before = before.into_iter().rev().collect::<Vec<(isize,Song)>>();
        let current = before.pop().unwrap().1;

        // let current = backend.audio.queue.get(&backend.audio.idx).unwrap().clone(); // TODO: error handling for empty queue?
        
        let after = backend.audio.queue.iter_from(&backend.audio.idx)
            .enumerate()
            .map(|(a,b)| (a as isize, b.clone()))
            .skip(1)
            .collect::<Vec<(isize, Song)>>();


        // let mut songs = Vec::with_capacity(before.len() + after.len() + 1);
        let mut songs = before;
        songs.push((0,current));
        for s in after {
            songs.push(s)
        }
        // dbg!(&songs);

        let songs = songs.into_iter().map(|(offset, song)| QUEUE_LINE_VIEW_BUILDER.build_with_msg(&song, Message::Skip(offset))).collect();

        let col = Column::from_vec(songs)
            .width(Length::FillPortion(1));
        scrollable(col)
            .into()
    }
}
impl Playing {
    // TODO: fix jank
    fn check_backend_updates(&mut self, backend: &mut backend::Resonance, current_message: &PlayingMessage) -> Task {
        let mut task = Task::none();
        if backend.audio.song_refresh_pending && *current_message != PlayingMessage::SongUpdate {
            task = self.update_song(backend.audio.current_song.clone());
            backend.audio.song_refresh_pending = false;
        }
        task
    }

    fn update_song(&mut self, song: Option<Song>) -> Task {
        if song.is_none() {
            todo!("no song playing in playing screen")
        }
        let song = song.unwrap();
        const BLUR_SIGMA: f32 = 75.0;

        self.pos = 0.0;
        self.song = song;
        let img = backend::blur(&self.song.ytid, BLUR_SIGMA);
        self.bg.copy_from_slice(&img);
        Task::none()
    }

    fn position_update(&mut self, pos: f32) -> Task {
        self.pos = pos;
        Task::none()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlayingMessage {
    SongUpdate, // When song changes
    PlaybackUpdate,
    PositionUpdate,
    QueueShown(bool),
}


const QUEUE_LINE_VIEW_BUILDER: crate::widgets::song::line_view::Builder = crate::widgets::song::line_view::Builder {
    // TODO: move around in queue
    cover_click_message: |s| Message::PlaySong(s.clone()),
    background: None,
    show_queue_button: false,
    alignment: Horizontal::Right,
    image_side: true,
};
