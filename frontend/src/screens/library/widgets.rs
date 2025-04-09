use crate::Message;
use backend::Song;
use crate::appearance::{colours, styles};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{ button, column, container, hover, image, row, svg, text, Space };
use iced::{Background, Border, Element, Theme, Fill};

pub fn song<'a>(song: &Song) -> Element<'a, crate::Message> {
    const THUMBNAIL_SIZE: u32 = 48;

    let thumbnail = container(image(backend::dirs().song_thumbnail(&song.ytid))
        .height(THUMBNAIL_SIZE))
        .align_y(Vertical::Center);

    let play_icon = container(svg(crate::assets::icon()).width(Fill).height(Fill))
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
        .on_press(Message::Queue(backend::QueueEvent::AddToEnd(song.clone())));
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


    let song_info = column![
        title,
        row![
            artist,
            text("·"),
            album,
            text("·"),
            duration,
        ].spacing(5),
    ].spacing(5);

    let contents = row![thumbnail_overlay, song_info]
        .spacing(10)
        .align_y(Vertical::Center);

    // Container style
    let container = container(contents)
        .style(|_theme: &Theme| {
            container::Style::default()
                .background(Background::Color(colours::SURFACE0))
                .border(Border::default().rounded(10))
        })
        .padding(5)
        .width(Fill)
        .align_x(Horizontal::Left);

    return Element::new(container);
}
