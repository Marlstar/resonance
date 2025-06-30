use iced::widget;
use iced::Length;
use iced::Length::Fill;
use crate::models::Song;
use crate::iced::types::Element;
use crate::dirs;
use crate::daemon::Daemon;
use crate::daemon::Message;
use crate::util::millis_to_formatted_duration;

const COVER_SIZE: u32 = 300;

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
            let image = widget::image(dirs::cover::yt(ytid));
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
        let playpause = widget::button("playpause");
        let skip_next = widget::button("next");
        let skip_prev = widget::button("prev");

        // ====== \\
        // LAYOUT \\
        // ====== \\

        let info = widget::column![
            title,
            artist,
            position,
        ];

        let info_with_cover = widget::row![
            cover,
            info,
        ];

        let controls = widget::row![
            skip_prev,
            playpause,
            skip_next,
        ];

        let everything = widget::column![
            info_with_cover,
            progress,
            controls,
        ].width(Length::Shrink);

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
            self.artist = None;
            self.album = None;
            self.song = Some(song);
        } else {
            self.song = None;
            self.album = None;
            self.artist = None;
        }
    }
}
