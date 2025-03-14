use diesel::prelude::*;

#[derive(Debug, Clone)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::playlist)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Playlist {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::playlist)]
pub struct NewPlaylist<'a> {
    pub name: &'a str
}

#[allow(dead_code)]
pub fn create(conn: &mut SqliteConnection, name: &str) -> Result<Playlist, diesel::result::Error> {
    use crate::db::schema::playlist;

    let new_playlist = NewPlaylist {
        name,
    };

    diesel::insert_into(playlist::table)
        .values(&new_playlist)
        .returning(Playlist::as_returning())
        .get_result(conn)
}
