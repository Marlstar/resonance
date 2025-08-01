use std::path::PathBuf;
use diesel::{insert_into, prelude::*};
use crate::daemon::Message;
use crate::db::schema::songs;
use crate::db::pool;

#[derive(Debug, Clone, PartialEq)]
#[derive(Queryable, Selectable, Identifiable, AsChangeset)]
#[diesel(table_name = crate::db::schema::songs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Song {
    pub id: i32,
    pub ytid: Option<String>,
    pub name: String,
    pub artist: Option<i32>,
    pub album: Option<i32>,
    pub duration: i32,
    pub downloaded: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::songs)]
pub struct NewSong<'a> {
    pub ytid: Option<&'a str>,
    pub name: &'a str,
    pub artist: Option<i32>,
    pub album: Option<i32>,
    pub duration: i32,
    pub downloaded: bool,
}

impl Song {
    pub fn create(
        ytid: Option<&str>,
        name: &str,
        artist: Option<i32>,
        album: Option<i32>,
        duration: i32
    ) -> Result<Song, diesel::result::Error> {
        use crate::db::schema::songs;

        let new_song = NewSong {
            ytid,
            name,
            artist,
            album,
            duration,
            downloaded: false,
        };

        insert_into(songs::table)
            .values(&new_song)
            .returning(Song::as_returning())
            .get_result(&mut pool::get())
    }

    pub fn download(&self) -> Message {
        if self.ytid.is_some() {
            Message::DownloadSong(self.clone())
        } else { Message::None }
    }

    pub fn path(&self) -> PathBuf {
        if let Some(id) = &self.ytid {
            return crate::dirs::song::yt(id);
        }
        todo!("non-yt song path")
    }

    pub fn search(
        name: &str,
        artist: Option<i32>,
        album: Option<i32>,
    ) -> Result<Option<Song>, diesel::result::Error> {
        songs::table
            .filter(songs::name.like(format!("%{name}%")))
            .filter(songs::artist.eq(artist))
            .filter(songs::album.eq(album))
            .first(&mut pool::get())
            .optional()
    }

    pub fn get(id: i32) -> Result<Option<Self>, diesel::result::Error> {
        songs::table
            .filter(songs::id.eq(id))
            .first(&mut pool::get())
            .optional()
    }

    pub fn push_updates(&self) -> Result<(), diesel::result::Error> {
        diesel::update(songs::table)
            .filter(songs::id.eq(self.id))
            .set(self)
            .execute(&mut pool::get())
            .map(|_| ())
    }
}
