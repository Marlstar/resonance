use diesel::prelude::*;
use crate::db::schema::albums;
use crate::db::pool;

#[derive(Debug, Clone, PartialEq)]
#[derive(Queryable, Selectable, Identifiable, AsChangeset)]
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

impl Album {
    pub fn create(
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

        diesel::insert_into(albums::table)
            .values(&new_album)
            .returning(Album::as_returning())
            .get_result(&mut pool::get())
    }

    pub fn search(
        name: &str,
        artist: Option<i32>,
    ) -> Result<Option<Album>, diesel::result::Error> {
        albums::table
            .filter(albums::name.eq(name))
            .filter(albums::artist.eq(artist))
            .first(&mut pool::get())
            .optional()
    }

    pub fn get_or_create(name: &str, artist: Option<i32>) -> Result<Album, diesel::result::Error> {
        if let Some(album) = Self::search(name, artist)? {
            return Ok(album);
        }
        return Self::create(name, artist, 0);
    }

    pub fn get(id: i32) -> Result<Option<Self>, diesel::result::Error> {
        albums::table
            .filter(albums::id.eq(id))
            .first(&mut pool::get())
            .optional()
    }

    pub fn push_updates(&self) -> Result<(), diesel::result::Error> {
        diesel::update(albums::table)
            .filter(albums::id.eq(self.id))
            .set(self)
            .execute(&mut pool::get())
            .map(|_| ())
    }
}
