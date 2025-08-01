use crate::daemon::Message;
use crate::iced::types::Element;
use crate::models::*;
use iced::alignment::{Horizontal, Vertical};
use iced::widget;
use iced::{Border, Fill};

const THUMBNAIL_SIZE: u32 = 48;

pub struct Builder {
    pub cover_click_message: fn(Song) -> Message,
    pub background: Option<iced::Background>,
}
impl Builder {
    pub fn build<'a>(&self, song: Song) -> Element<'a> {
        self.build_with_msg(song.clone(), (self.cover_click_message)(song))
    }

    pub fn build_with_msg<'a>(&self, song: Song, msg: Message) -> Element<'a> {
        println!("building song {}", song.name);
        let thumbnail = widget::container(widget::image(crate::dirs::cover::yt(song.ytid.as_ref().unwrap()))
            .height(THUMBNAIL_SIZE))
            .align_y(Vertical::Center);

        let play_icon = widget::container(crate::assets::icon().width(Fill).height(Fill))
            .center(Fill);
        let play_button = widget::button(play_icon)
            .width(Fill)
            .height(Fill)
            .on_press(msg);
        let play_overlay = widget::container(play_button)
            .style(widget::container::transparent)
            .center(Fill);
        let thumbnail_overlay = widget::hover(thumbnail, play_overlay);

        let title = widget::text(song.name.clone())
            .size(22);
        let artist = widget::text(song.artist.and_then(|a| a.get_artist().map(|a| a.name)).unwrap_or("No artist".to_string()))
            // .style(styles::grey_text)
            .size(16);
        let album = widget::text(song.album.and_then(|a| a.get_artist().map(|a| a.name)).unwrap_or("No album".to_string()))
            // .style(styles::grey_text)
            .size(16);
        let duration = widget::text(crate::util::millis_to_formatted_duration(song.duration))
            // .style(styles::grey_text)
            .size(16);

        let queue_end_button = widget::button(crate::assets::queue_end().width(30.0).height(30.0))
            .style(|_,_| widget::button::Style::default())
            // .on_press(Message::Queue(backend::QueueEvent::AddToEnd(song.clone())));
            .on_press(Message::None);

        let song_info = widget::column![
            title,
            widget::row![
                artist,
                widget::text("·"),
                album,
                widget::text("·"),
                duration,
                widget::text("·"),
                widget::text(song.ytid.as_ref().unwrap().clone()),
            ].spacing(5),
        ].spacing(5).align_x(Horizontal::Left);

            let contents = widget::row![thumbnail_overlay, song_info, widget::Space::new(Fill, 0.0), queue_end_button]
                .spacing(10)
                .align_y(Vertical::Center);

        // Container style
        let bg = self.background;
        let container = widget::container(contents)
            .style(move |_| {
                let mut style = widget::container::Style::default()
                    .border(Border::default().rounded(10));
                if let Some(bg) = bg {
                    style.background = Some(bg);
                }
                style
            })
            .padding(5)
            .width(Fill)
            .align_x(Horizontal::Left);

        // return Element::new(container.style(|_| widget::container::Style{background:Some(iced::Background::Color(iced::color!(0xffffff))),..Default::default()}));
        return Element::new(container);
    }
}
