use diesel::prelude::*;
use crate::db::schema::albums;
use crate::db::handler::DBHandler;

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
        db: &mut DBHandler,
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
            .get_result(&mut db.db)
    }

    pub fn search(
        db: &mut DBHandler,
        name: &str,
        artist: Option<i32>,
    ) -> Result<Option<Album>, diesel::result::Error> {
        albums::table
            .filter(albums::name.eq(name))
            .filter(albums::artist.eq(artist))
            .first(&mut db.db)
            .optional()
    }

    pub fn get_or_create(db: &mut DBHandler, name: &str, artist: Option<i32>) -> Result<Album, diesel::result::Error> {
        if let Some(album) = Self::search(db, name, artist)? {
            return Ok(album);
        }
        return Self::create(db, name, artist, 0);
    }

    pub fn push_updates(&self, db: &mut DBHandler) -> Result<(), diesel::result::Error> {
        diesel::update(albums::table)
            .filter(albums::id.eq(self.id))
            .set(self)
            .execute(&mut db.db)
            .map(|_| ())
    }
}
