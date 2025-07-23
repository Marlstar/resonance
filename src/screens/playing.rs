use iced::alignment::Horizontal;
use iced::alignment::Vertical;
use iced::widget;
use iced::Length;
use iced::Length::Fill;
use crate::models::*;
use crate::iced::types::Element;
use crate::dirs;
use crate::daemon::Daemon;
use crate::daemon::Message;
use crate::util::millis_to_formatted_duration;
use crate::assets;

const COVER_SIZE: u32 = 240;

pub struct PlayingScreen {
    song: Option<Song>,
    artist: Option<String>,
    album: Option<String>,
}
impl Default for PlayingScreen {
    fn default() -> Self {
        return Self {
            song: None,
            artist: None,
            album: None,
        };
    }
}
impl PlayingScreen {
    pub fn fullscreen(&self, daemon: &Daemon) -> Element {
        let ytid = self.song.as_ref()
            .and_then(|song| song.ytid.as_ref());
        let cover = if let Some(ytid) = ytid {
            let image = widget::image(dirs::cover::yt(ytid)).width(COVER_SIZE).height(COVER_SIZE);
            Element::new(image)
        } else {
            let space = widget::Space::new(COVER_SIZE, COVER_SIZE);
            Element::new(space)
        };

        let title = match &self.song {
            Some(song) => widget::text(&song.name),
            None => widget::text("Nothing playing")
        }.size(22).font(*crate::fonts::BOLD);

        let artist = match &self.artist {
            Some(artist) => widget::text(artist),
            None => widget::text("No artist"),
        };

        // let album = match &self.album {
        //     Some(album) => widget::text(album),
        //     None => widget::text("No album"),
        // };

        let pos = daemon.audio.position;
        let dur = self.song.as_ref().map(|s| s.duration).unwrap_or(0);

        let position = millis_to_formatted_duration(pos);
        let duration = millis_to_formatted_duration(dur);
        let position = widget::text(format!("{position}/{duration}"));

        let progress = widget::slider(0..=dur, pos, |_| Message::None);

        // ======== \\
        // CONTROLS \\
        // ======== \\
        fn containerise_svg(s: widget::Svg, width: u32, height: u32) -> widget::Container<Message> {
            widget::container(s).width(width).height(height)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
        }
        let playpause = widget::button(
            containerise_svg(
                if daemon.audio.playing {assets::pause()} else {assets::play()},
                60, 60
            )
        ).width(60).height(60)
            .style(|_,_| widget::button::Style::default()); // Transparent bg

        let _skip_size = 55;
        let skip_next = widget::button(containerise_svg(assets::skip_forward(), _skip_size, _skip_size))
            .width(_skip_size).height(_skip_size)
            .style(|_,_| widget::button::Style::default()); // Transparent bg
        let skip_prev = widget::button(containerise_svg(assets::skip_back(), _skip_size, _skip_size))
            .width(_skip_size).height(_skip_size)
            .style(|_,_| widget::button::Style::default()); // Transparent bg

        // ====== \\
        // LAYOUT \\
        // ====== \\
        let info = widget::column![
            title,
            artist,
            position,
        ].spacing(20);

        let info_with_cover = widget::row![
            cover,
            info,
        ].spacing(10);

        let controls = widget::row![
            widget::horizontal_space(),
            skip_prev,
            playpause,
            skip_next,
            widget::horizontal_space(),
        ].align_y(Vertical::Center);

        let everything = widget::column![
            info_with_cover,
            progress,
            controls,
        ].width(Length::Shrink).spacing(10);

        let centered = widget::container(everything)
            .center_x(Fill)
            .center_y(Fill);

        return centered.into();
    }
}
impl PlayingScreen {
    pub fn update_song(&mut self, song: Option<Song>) {
        if let Some(song) = song {
            // TODO: get artist and album
            self.artist = song.artist.and_then(|a| a.get_artist()).map(|a| a.name);
            self.album = song.album.and_then(|a| a.get_album()).map(|a| a.name);
            self.song = Some(song);
        } else {
            self.song = None;
            self.album = None;
            self.artist = None;
        }
    }
}
