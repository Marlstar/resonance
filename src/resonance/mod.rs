use crate::{Error, Database, Song};
use youtube_dl::SingleVideo;

mod download;
mod search;
use search::{search_youtube_for_ids, get_full_metadata};

pub struct Resonance {
    db: Database
}
impl Resonance {
    pub fn new() -> Result<Self, Error> {
        return Ok(Self {
            db: Database::load()?
        });
    }
}
impl Resonance {
    pub fn run(&mut self) {
    }

    pub fn exit(self) {
        self.db.close();
    }

    pub fn download(&mut self, url: &str) -> Result<Song, Error> {
        let ytid = match crate::util::get_ytid_from_url(url) {
            Some(a) => a,
            None => return Err(Error::InvalidURL)
        };

        if self.db.ytid_is_used(&ytid)? {
            return Err(Error::SongAlreadyInstalled);
        }

        print!("Downloading song ({ytid}) ");
        crate::util::flush_stdout();

        let vid = self.download_song(url)?;
        println!("| DONE!");

        let name = vid.title.expect("failed to get video title");
        let author = vid.channel.expect("failed to get video channel");
        let duration = vid.duration.expect("failed to get video duration").as_i64().unwrap() as i32;

        self.db.add_song(&ytid, &name, &author, duration)
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
