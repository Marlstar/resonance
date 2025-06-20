use diesel::{insert_into, prelude::*};
use crate::db::handler::DBHandler;
use crate::db::schema::songs;

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
        conn: &mut DBHandler,
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
            .get_result(&mut conn.db)
    }

    pub fn push_updates(&self, db: &mut DBHandler) -> Result<(), diesel::result::Error> {
        diesel::update(songs::table)
            .filter(songs::id.eq(self.id))
            .set(self)
            .execute(&mut db.db)
            .map(|_| ())
    }
}
