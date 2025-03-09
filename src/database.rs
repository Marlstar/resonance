use diesel::prelude::*;
use diesel::{ConnectionError, SqliteConnection, result::DatabaseErrorKind};
use crate::Song;
use crate::Error;

pub struct Database {
    db: SqliteConnection
}
impl Database { // Setup
    pub fn load() -> Result<Self, ConnectionError> {
        use crate::db::connect;
        let connection = connect()?;
        return Ok(Self {
            db: connection
        });
    }

    pub fn close(self)  {
        return;
    }

    pub fn backup(&self) -> Result<(), Error> {
        let current_save_path = crate::dirs().db();
        let backup_path = crate::dirs().db_backup();
        return match std::fs::copy(current_save_path, backup_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::BackupFailed(e))
        };
    }
}
impl Database { // Songs
    pub fn add_song(&mut self, ytid: &str, name: &str, author: &str, duration: i32) -> Result<Song, Error> {
        if self.ytid_is_used(ytid)? {
            return self.get_song_by_ytid(ytid);
        }

        return match crate::models::song::create(&mut self.db, ytid, name, author, duration) {
            Ok(song) => Ok(song),
            // This match arm shouldn't be reached, should return early if this is the case
            Err(diesel::result::Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => self.get_song_by_ytid(ytid),
            Err(e) => Err(e.into())
        };
    }

    pub fn rename_song(&mut self, song_id: i32, new_name: &str) -> Result<Song, Error> {
        use crate::db::schema::song::dsl::*;

        return Ok(diesel::update(song.find(song_id))
            .set(name.eq(new_name))
            .returning(Song::as_returning())
            .get_result(&mut self.db)?);
    }

    pub fn rename_song_by_ytid(&mut self, id: &str, new_name: &str) -> Result<Song, Error> {
        use crate::db::schema::song::dsl::{song, ytid, name};

        return Ok(diesel::update(song.filter(ytid.like(id)))
            .set(name.eq(new_name))
            .returning(Song::as_returning())
            .get_result(&mut self.db)?);
    }

    pub fn delete_song(&mut self, id: i32) -> Result<(), Error> {
        use crate::db::schema::song::dsl::song;

        diesel::delete(song.find(id))
            .execute(&mut self.db)?;

        return Ok(());
    }

    pub fn get_song(&mut self, id: i32) -> Result<Song, Error> {
        use crate::db::schema::song;

        let s = song::table
            .find(id)
            .first(&mut self.db)
            .optional();

        return match s {
            Ok(Some(song)) => Ok(song),
            Ok(None) => Err(Error::SongNotInstalled),
            Err(e) => Err(e.into())
        };
    }

    pub fn get_song_by_ytid(&mut self, id: &str) -> Result<Song, Error> {
        use crate::db::schema::song::dsl::{song, ytid};

        let s = song
            .filter(ytid.eq(id))
            .first(&mut self.db)
            .optional();

        return match s {
            Ok(Some(a)) => Ok(a),
            Ok(None) => Err(Error::SongNotInstalled),
            Err(e) => Err(e.into())
        };
    }


    pub fn ytid_is_used(&mut self, id: &str) -> Result<bool, Error> {
        use crate::db::schema::song::dsl::{song, ytid};

        let s: Option<Song> = song.filter(ytid.eq(id)).first(&mut self.db).optional()?;
        return Ok(s.is_some());
    }

    pub fn get_all_songs(&mut self) -> Result<Vec<Song>, Error> {
        use crate::db::schema::song::dsl::song;

        return Ok(song.load(&mut self.db)?);
    }

    pub fn search_songs(&mut self, query: &str) -> Result<Vec<Song>, Error> {
        use crate::db::schema::song::dsl::*;

        return Ok(song
            .filter(name.like(format!("%{query}%")))
            .load(&mut self.db)?);
    }
}
