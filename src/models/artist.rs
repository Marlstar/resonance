use diesel::{insert_into, prelude::*};

#[derive(Debug, Clone, PartialEq)]
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::db::schema::artists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Artist {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::db::schema::artists)]
pub struct NewArtist<'a> {
    pub name: &'a str,
}

pub fn create(
    conn: &mut SqliteConnection,
    name: &str,
) -> Result<Artist, diesel::result::Error> {
    use crate::db::schema::artists;

    let new_artist = NewArtist {
        name,
    };

    insert_into(artists::table)
        .values(&new_artist)
        .returning(Artist::as_returning())
        .get_result(conn)
}
