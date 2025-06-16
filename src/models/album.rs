use diesel::{insert_into, prelude::*};

#[derive(Debug, Clone, PartialEq)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::albums)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Album {
    pub id: i32,
    pub name: String,
    pub artist: Option<i32>,
    pub length: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::albums)]
pub struct NewAlbum<'a> {
    pub name: &'a str,
    pub artist: Option<i32>,
    pub length: i32,
}

pub fn create(
    conn: &mut SqliteConnection,
    name: &str,
    artist: Option<i32>,
    length: i32,
) -> Result<Album, diesel::result::Error> {
    use crate::db::schema::albums;

    let new_album = NewAlbum {
        name,
        artist,
        length,
    };

    insert_into(albums::table)
        .values(&new_album)
        .returning(Album::as_returning())
        .get_result(conn)
}
