use crate::Message;
use crate::backend::Song;
use crate::appearance::colours;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{ button, column, container, hover, image, svg, row, text };
use iced::{Background, Border, Element, Length, Theme};

pub fn song<'a>(song: &Song) -> Element<'a, crate::Message> {
    let grey_text = |_: &Theme| -> text::Style {
        text::Style {
            color: Some(colours::OVERLAY2)
        }
    };
    const THUMBNAIL_SIZE: u32 = 48;

    let thumbnail = container(image(crate::backend::dirs().song_thumbnail(&song.ytid))
        .height(THUMBNAIL_SIZE))
        .align_y(Vertical::Center);

    let play_icon = container(svg(crate::assets::icon_svg()).width(Length::Fill).height(Length::Fill))
        .center(Length::Fill);
    let play_button = button(play_icon)
        .style(|_: &Theme, _: button::Status| {
            button::Style {
                background: Some(Background::Color(colours::HOVER)),
                ..Default::default()
            }
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .on_press(Message::PlaySong(song.id));
    let play_overlay = container(play_button)
        .center(Length::Fill);
    let thumbnail_overlay = hover(thumbnail, play_overlay);

    let title = text(song.name.clone())
        .size(22);
    let artist = text(song.author.clone())
        .style(grey_text)
        .size(16);
    let album = text(song.album.clone())
        .style(grey_text)
        .size(16);
    let duration = text(format!("{}:{}", (song.duration - (song.duration % 60))/60, song.duration % 60))
        .style(grey_text)
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
        .padding(5);

    return Element::new(container);
}
