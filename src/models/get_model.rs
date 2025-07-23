use super::{Song, Artist, Album};

pub trait GetSongExt { fn get_song(self) -> Option<Song>; }
pub trait GetArtistExt { fn get_artist(self) -> Option<Artist>; }
pub trait GetAlbumExt { fn get_album(self) -> Option<Album>; }

impl GetSongExt for i32 {
    fn get_song(self) -> Option<Song> {
        Song::get(self).ok()?
    }
}
impl GetArtistExt for i32 {
    fn get_artist(self) -> Option<Artist> {
        Artist::get(self).ok()?
    }
}
impl GetAlbumExt for i32 {
    fn get_album(self) -> Option<Album> {
        Album::get(self).ok()?
    }
}
