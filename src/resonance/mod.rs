use crate::{Error, Database, Song};

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

        let (vid, path) = self.download_song(url)?;
        println!("| DONE!");

        let name = vid.title.expect("failed to get video title");
        let author = vid.channel.expect("failed to get video channel");
        let path_str = crate::util::path_to_string(&path);
        let duration = vid.duration.expect("failed to get video duration").as_i64().unwrap() as i32;

        self.db.add_song(&ytid, &name, &author, &path_str, duration)
    }

    pub fn search(&self, query: &str, count: usize) -> Result<(), Error> {
        if count == 0 { return Err(Error::NoSearchResults); }

        let ids = search_youtube_for_ids(query)?;
        if ids.is_empty() { return Err(Error::NoSearchResults); }
        // TODO: when frontend made, update straight away with available info and fill in later using a task
        for id in ids.iter().take(count) {
            let song = get_full_metadata(id)?;
            println!("| {}", song.title.unwrap());
        }

        return Ok(());
    }

    pub fn delete(&mut self, id: i32) -> Result<(), Error> {
        let song = self.db.get_song(id)?;
        self.db.delete_song(id)?;
        // TODO: when thumbnails implemented, remove thumbnail file
        let _ = std::fs::remove_file(crate::dirs().song_file(&song.ytid));
        let _ = std::fs::remove_dir(crate::dirs().song(&song.ytid));
        return Ok(());
    }

    pub fn rename(&mut self, id: i32, name: String) -> Result<(), Error> {
        self.db.rename_song(id, &name)?;
        return Ok(());
    }

    pub fn rename_by_ytid(&mut self, id: &str, new_name: &str) -> Result<Song, Error> {
        self.db.rename_song_by_ytid(id, new_name)
    }

    pub fn list_songs(&mut self) -> Result<(), Error> {
        let songs = self.db.get_all_songs()?;
        for song in songs {
            println!("| {}", song.name);
        }
        return Ok(());
    }
}
