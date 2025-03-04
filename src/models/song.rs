use diesel::{insert_into, prelude::*};

#[derive(Debug, Clone, PartialEq)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::song)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Song {
    pub id: i32,
    pub ytid: String,
    pub name: String,
    pub author: String,
    pub path: String,
    pub duration: i32
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::song)]
pub struct NewSong<'a> {
    pub ytid: &'a str,
    pub name: &'a str,
    pub author: &'a str,
    pub path: &'a str,
    pub duration: i32
}

pub fn create(conn: &mut SqliteConnection, ytid: &str, name: &str, author: &str, path: &str, duration: i32) -> Result<Song, diesel::result::Error> {
    use crate::db::schema::song;

    let new_song = NewSong {
        ytid,
        name,
        author,
        path,
        duration
    };

    insert_into(song::table)
        .values(&new_song)
        .returning(Song::as_returning())
        .get_result(conn)
}
