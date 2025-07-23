use diesel::prelude::*;
use crate::db::schema::artists;
use crate::db::pool;


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

impl Artist {
    pub fn create(name: &str) -> Result<Artist, diesel::result::Error> {
        let new_artist = NewArtist {
            name,
        };

        diesel::insert_into(artists::table)
            .values(&new_artist)
            .returning(Artist::as_returning())
            .get_result(&mut pool::get())
    }

    pub fn search(name: &str) -> Result<Option<Artist>, diesel::result::Error> {
        artists::table
            .filter(artists::name.eq(name))
            .first(&mut pool::get())
            .optional()
    }

    pub fn get(id: i32) -> Result<Option<Self>, diesel::result::Error> {
        artists::table
            .filter(artists::id.eq(id))
            .first(&mut pool::get())
            .optional()
    }

    pub fn get_or_create(name: &str) -> Result<Artist, diesel::result::Error> {
        if let Some(artist) = Self::search(name)? {
            return Ok(artist);
        }
        return Self::create(name);
    }
}
