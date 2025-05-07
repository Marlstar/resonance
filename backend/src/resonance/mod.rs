use std::thread::JoinHandle;
use hashbrown::HashSet;

use crate::{AudioPlayer, Database, Error, Song};
use youtube_dl::SingleVideo;

mod search;
use search::{search_youtube_for_ids, get_full_metadata};

pub struct Resonance {
    db: Database,
    pub audio: AudioPlayer,
    pub downloading: HashSet<String>,
    _mpris_handle: JoinHandle<()>,
}
impl Resonance {
    pub fn new() -> Result<Self, Error> {
        let s = Self {
            db: Database::load()?,
            audio: AudioPlayer::new()?,
            downloading: HashSet::new(),
            _mpris_handle: crate::mpris::run(),
        };
        Ok(s)
    }
}
impl Resonance {
    pub fn exit(self) {
        self.db.close();
    }

    pub fn install_downloaded(&mut self, vid: SingleVideo) -> Result<Song, Error> {
        if self.db.ytid_is_used(&vid.id)? {
            return Err(Error::SongAlreadyInstalled);
        }

        let id = vid.id;
        let name = vid.title.expect("failed to get video title");
        let mut author = vid.channel.expect("failed to get video channel");
        if let Some(notopic) = author.strip_suffix(" - Topic") {
            author = notopic.to_string();
        }
        let album = vid.album.unwrap_or("No album".to_string());
        // TODO: get higher-accuracy duration from audio file directly
        let duration = vid.duration.expect("failed to get video duration").as_i64().unwrap() as i32;

        self.db.add_song(&id, &name, &author, &album, duration)
    }

    pub fn get_song(&mut self, id: i32) -> Result<Song, Error> {
        self.db.get_song(id)
    }

    pub fn search(&self, query: &str, count: usize) -> Result<Vec<SingleVideo>, Error> {
        if count == 0 { return Err(Error::NoSearchResults); }

        let ids = search_youtube_for_ids(query)?;
        if ids.is_empty() { return Err(Error::NoSearchResults); }
        // TODO: when frontend made, update straight away with available info and fill in later using a task
        let mut songs: Vec<SingleVideo> = Vec::with_capacity(count);

        for id in ids.iter().take(count) {
            match get_full_metadata(id) {
                Ok(s) => songs.push(s),
                Err(_) => continue
            };
        }

        return Ok(songs);
    }

    pub fn delete(&mut self, id: i32) -> Result<(), Error> {
        let song = self.db.get_song(id)?;
        self.db.delete_song(id)?;
        let _ = std::fs::remove_file(crate::dirs().song_file(&song.ytid));
        let _ = std::fs::remove_file(crate::dirs().song_thumbnail(&song.ytid));
        // Shouldn't still exist, but remove it in case
        let _ = std::fs::remove_file(crate::dirs().song_thumbnail_uncropped(&song.ytid));
        let _ = std::fs::remove_dir(crate::dirs().song(&song.ytid));
        return Ok(());
    }

    pub fn rename(&mut self, id: i32, name: String) -> Result<Song, Error> {
        self.db.rename_song(id, &name)
    }

    pub fn rename_by_ytid(&mut self, id: &str, new_name: &str) -> Result<Song, Error> {
        self.db.rename_song_by_ytid(id, new_name)
    }

    pub fn list_songs(&mut self) -> Result<Vec<Song>, Error> {
        self.db.get_all_songs()
    }
}
