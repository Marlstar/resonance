use crate::backend::Song;
use crate::appearance::colours;
use iced::alignment::Vertical;
use iced::widget::{ column, container, image, row, text };
use iced::{Background, Border, Element, Theme};

pub fn song<'a>(song: &Song) -> Element<'a, crate::Message> {
    let grey_text = |_: &Theme| -> text::Style {
        text::Style {
            color: Some(colours::OVERLAY2)
        }
    };

    let thumbnail = image(crate::backend::dirs().song_thumbnail(&song.ytid))
        .height(50);
    let thumbnail = container(thumbnail)
        .padding(0)
        .style(|_: &Theme| {
            container::Style::default()
                .border(Border::default().rounded(6))
        });
    let title = text(song.name.clone())
        .size(22);
    let artist = text(song.author.clone())
        .style(grey_text)
        .size(16);
    let duration = text(format!("{}s", song.duration))
        .style(grey_text)
        .size(16);

    // TODO: album

    let song_info = column![
        title,
        row![
            artist,
            text("·"),
            duration,
        ].spacing(5),
    ].spacing(5);

    let contents = row![thumbnail, song_info]
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
