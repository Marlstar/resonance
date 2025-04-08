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
    pub duration: i32,
    pub album: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::song)]
pub struct NewSong<'a> {
    pub ytid: &'a str,
    pub name: &'a str,
    pub author: &'a str,
    pub duration: i32,
    pub album: &'a str,
}

pub fn create(conn: &mut SqliteConnection, ytid: &str, name: &str, author: &str, album: &str, duration: i32) -> Result<Song, diesel::result::Error> {
    use crate::db::schema::song;

    let new_song = NewSong {
        ytid,
        name,
        author,
        album,
        duration
    };

    insert_into(song::table)
        .values(&new_song)
        .returning(Song::as_returning())
        .get_result(conn)
}

// TODO: fix this jank after queueing works fully
impl Song {
    #[allow(non_snake_case)]
    pub fn NONE() -> Song {
        Song {
            id: -1,
            name: String::with_capacity(1),
            ytid: "".to_string(),
            author: "".to_string(),
            duration: 0,
            album: "".to_string(),
        }
    }
    #[allow(non_snake_case)]
    pub fn IS_NONE(&self) -> bool {
        return self.id == -1;
    }
}
