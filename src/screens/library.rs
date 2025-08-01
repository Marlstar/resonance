use crate::daemon::Message;
use crate::models::*;
use crate::iced::types::Element;
use crate::widgets;
use iced::widget;


pub struct LibraryScreen {
    songs: Vec<Song>,
}
impl Default for LibraryScreen {
    fn default() -> Self {
        LibraryScreen {
            songs: Song::all().unwrap(),
        }
    }
}
impl LibraryScreen {
    pub fn view(&self) -> Element {
        let songs = self.songs.iter().map(|s| SONG_WIDGET_BUILDER.build(s.clone()));
        return widget::Column::from_iter(songs).into();
    }
}

static SONG_WIDGET_BUILDER: widgets::song::Builder = widgets::song::Builder {
    cover_click_message: Message::LoadSong,
    background: None,
};
