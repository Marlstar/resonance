use std::io::Read;

use serde::{Serialize, Deserialize};
use hashbrown::HashSet;
use crate::Song;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct State {
    pub songs: HashSet<Song>
}
impl State { // Init stuff
    pub fn new() -> Self {
        Default::default()
    }
    
    pub fn load() -> Result<Self, StateError> {
        let mut buf = String::new();
        std::fs::File::open(crate::dirs().state())?.read_to_string(&mut buf)?;
        return Ok(ron::from_str::<State>(buf.as_str())?);
    }

    pub fn save(&self) -> Result<(), StateError> {
        let ser = ron::to_string(self)?;
        // Create state backup
        'backup: {
            let current_save_path = crate::dirs().state();
            let backup_path = crate::dirs().state_backup();

            match std::fs::copy(current_save_path, backup_path) {
                Ok(_) => (),
                Err(_) => break 'backup // No save to backup
            }
        }

        std::fs::write(crate::dirs().state(), ser)?;
        return Ok(());
    }
}

impl State { // Songs
    pub fn add_song(&mut self, song: Song) -> Result<(), StateError> {
        return match self.songs.insert(song) {
            true => Ok(()),
            false => Err(StateError::SongAlreadyInstalled)
        };
    }

    pub fn remove_song(&mut self, id: &String) -> Result<(), StateError> {
        return match self.songs.remove(id) {
            true => Ok(()),
            false => Err(StateError::SongNotInstalled)
        };
    }

    pub fn get_song(&self, id: &String) -> Result<&Song, StateError> {
        return match self.songs.get(id) {
            Some(a) => Ok(a),
            None => Err(StateError::SongNotInstalled)
        };
    }

    pub fn song_is_installed(&self, id: &String) -> bool {
        return self.songs.get(id).is_some();
    }
}

#[derive(Debug, Clone)]
pub enum StateError {
    // Addition
    SongAlreadyInstalled,
    // Removal
    SongNotInstalled,

    // Loading/saving
    Serialisation(ron::Error),
    Deserialisation(ron::de::SpannedError),
    StateFileNotFound,
    IOError(std::io::ErrorKind),
}
impl From<std::io::Error> for StateError {
    fn from(value: std::io::Error) -> Self {
        return match value.kind() {
            std::io::ErrorKind::NotFound => StateError::StateFileNotFound,
            err => StateError::IOError(err)
        };
    }
}
impl From<ron::Error> for StateError {
    fn from(value: ron::Error) -> Self {
        return StateError::Serialisation(value)
    }
}
impl From<ron::de::SpannedError> for StateError {
    fn from(value: ron::de::SpannedError) -> Self {
        return StateError::Deserialisation(value)
    }
}
