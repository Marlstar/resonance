use crate::Message;
use backend::Song;
use crate::appearance::{colours, styles};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{ button, column, container, hover, image, row, text, Space };
use iced::{Background, Border, Element, Theme, Fill};

const THUMBNAIL_SIZE: u32 = 48;

pub struct Builder {
    pub cover_click_message: fn(&Song) -> Message,
    pub background: Option<iced::Background>,
    pub show_queue_button: bool,
    /// `false = left`, `true = right`
    pub alignment_x: bool,
}
impl Builder {
    pub fn build<'a>(&self, song: &Song) -> Element<'a, Message> {
        self.build_with_msg(song, (self.cover_click_message)(song))
    }

    pub fn build_with_msg<'a>(&self, song: &Song, msg: Message) -> Element<'a, Message> {
        let alignment = if self.alignment_x { Horizontal::Right } else { Horizontal::Left };

        let thumbnail = container(image(backend::dirs().song_thumbnail(&song.ytid))
            .height(THUMBNAIL_SIZE))
            .align_y(Vertical::Center);

        let play_icon = container(crate::assets::icon().width(Fill).height(Fill))
            .center(Fill);
        let play_button = button(play_icon)
            .style(|_: &Theme, _: button::Status| {
                button::Style {
                    background: Some(Background::Color(colours::HOVER)),
                    ..Default::default()
                }
            })
        .width(Fill)
            .height(Fill)
            // .on_press(Message::PlaySong(song.id));
            .on_press(msg);
        let play_overlay = container(play_button)
            .center(Fill);
        let thumbnail_overlay = hover(thumbnail, play_overlay);

        let title = text(song.name.clone())
            .size(22);
        let artist = text(song.author.clone())
            .style(styles::grey_text)
            .size(16);
        let album = text(song.album.clone())
            .style(styles::grey_text)
            .size(16);
        let duration = text(backend::util::format_duration(song.duration as usize))
            .style(styles::grey_text)
            .size(16);

        let queue_end_button = button(crate::assets::queue_end().width(30.0).height(30.0))
            .style(|_,_| button::Style::default())
            .on_press(Message::Queue(backend::QueueEvent::AddToEnd(song.clone())));

        let song_info = column![
            title,
            row![
                artist,
                text("·"),
                album,
                text("·"),
                duration,
            ].spacing(5),
        ].spacing(5).align_x(alignment);

            let mut contents = if self.alignment_x { // Right
                row![song_info, thumbnail_overlay]
            } else { // Left
                row![thumbnail_overlay, song_info]
            }
                .spacing(10)
                .align_y(Vertical::Center);
        if self.show_queue_button {
            contents = contents.push(Space::new(Fill, 0.0))
                .push(queue_end_button);
        }

        // Container style
        let bg = self.background;
        let container = container(contents)
            .style(move |_theme: &Theme| {
                let mut style = container::Style::default()
                    .border(Border::default().rounded(10));
                if let Some(bg) = bg {
                    style.background = Some(bg);
                }
                style
            })
            .padding(5)
            .width(Fill)
            .align_x(alignment);

        return Element::new(container);
    }
}
