use super::{Song, Artist, Album};

pub trait ModelsExt {
    fn get_song(self) -> Option<Song>;
    fn get_artist(self) -> Option<Artist>;
    fn get_album(self) -> Option<Album>;
}

impl ModelsExt for i32 {
    fn get_song(self) -> Option<Song> {
        Song::get(self).ok()?
    }
    fn get_artist(self) -> Option<Artist> {
        Artist::get(self).ok()?
    }
    fn get_album(self) -> Option<Album> {
        Album::get(self).ok()?
    }
}
