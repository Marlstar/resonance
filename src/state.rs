use crate::Song;
use rusqlite::Connection;

type IOError = std::io::Error;

#[derive(Debug)]
pub struct State {
    pub db: Connection
}
impl State { // Init stuff
    pub fn init() -> Result<Self, StateError> {
        return Ok(Self {
            db: Self::setup_db()?
        });
    }

    fn setup_db() -> Result<Connection, rusqlite::Error> {
        eprintln!("connecting to db");
        let db = Connection::open(crate::dirs().state())?;
        db.execute(
            "CREATE TABLE IF NOT EXISTS songs (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                url TEXT NOT NULL,
                path TEXT NOT NULL,
                duration INTEGER NOT NULL
            )",
            ()
        )?;

        return Ok(db);
    }

    pub fn backup(&self) -> Result<(), IOError> {
        let current_save_path = crate::dirs().state();
        let backup_path = crate::dirs().state_backup();

        std::fs::copy(current_save_path, backup_path)?;

        return Ok(());
    }
}

impl State { // Songs
    pub fn add_song(&mut self, song: Song) -> Result<(), StateError> {
        use rusqlite::ffi::{Error as FFIError, ErrorCode as FFIErrorCode};
        println!("adding song");
        return match self.db.execute(
            "INSERT INTO songs
            VALUES (?1, ?2, ?3, ?4, ?5)",
            (&song.id, &song.name, &song.url, &song.path, &song.duration)
        ) {
            Ok(0) => Err(StateError::SongAlreadyInstalled),
            Ok(_) => Ok(()),
            // If song already installed
            Err(rusqlite::Error::SqliteFailure(FFIError{
                code:FFIErrorCode::ConstraintViolation, extended_code:1555
            }, _)) => Err(StateError::SongAlreadyInstalled),
            // Propogate other errors
            Err(e) => Err(e.into())
        };
    }

    pub fn remove_song(&mut self, id: &String) -> Result<(), StateError> {
        return match self.db.execute("DELETE FROM songs WHERE id=?1", [id])? {
            0 => Err(StateError::SongNotInstalled),
            _ => Ok(()),
        };
    }

    pub fn get_song_by_id(&self, id: &str) -> Result<Song, StateError> {
        let mut query = self.db.prepare(format!("SELECT * FROM songs WHERE id='{id}'").as_str())?;
        let mut song_iter = query.query_map([], |row| {
            Ok(Song{
                id: row.get(0)?,
                name: row.get(1)?,
                url: row.get(2)?,
                path: row.get(3)?,
                duration: row.get(4)?
            })
        })?;

        return match song_iter.next() {
            Some(a) => Ok(a?),
            None => Err(StateError::SongNotInstalled)
        };
    }

    pub fn check_song_by_id(&self, id: &str) -> bool {
        return self.get_song_by_id(id).is_ok();
    }
}

#[derive(Debug)]
pub enum StateError {
    // Addition
    SongAlreadyInstalled,
    // Removal
    SongNotInstalled,

    // Loading/saving
    StateFileNotFound,
    IOError(std::io::ErrorKind),
    Rusqlite(rusqlite::Error)
}
impl From<std::io::Error> for StateError {
    fn from(value: std::io::Error) -> Self {
        return match value.kind() {
            std::io::ErrorKind::NotFound => StateError::StateFileNotFound,
            err => StateError::IOError(err)
        };
    }
}
impl From<rusqlite::Error> for StateError {
    fn from(value: rusqlite::Error) -> Self {
        return StateError::Rusqlite(value);
    }
}
