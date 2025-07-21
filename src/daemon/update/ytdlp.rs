use youtube_dl::SingleVideo;
use std::sync::Arc;
use crate::models::{Album, Artist, Song};
use crate::iced::types::Task;
use crate::daemon::Message;

impl super::super::Daemon {
    pub(super) fn get_song_metadata(&mut self, ytid: String) -> Task {
        return iced::Task::future(crate::jobs::metadata::song::yt(ytid.clone()))
            .map(move |r| Message::SongMetadata(ytid.clone(), Arc::new(r)));
    }

    pub(super) fn song_metadata_callback(&mut self, job_id: String, result: Arc<crate::Result<Box<SingleVideo>>>) -> Task {
        let song = match &*result {
            Ok(vid) => vid,
            Err(_) => {
                println!("[metadata] error getting metadata for job {job_id}");
                return Task::none();
            }
        };

        let artist = if let Some(a) = &song.artist {
            Artist::get_or_create(&mut self.db, a).ok().map(|a| a.id)
        } else { None };

        let album = if let Some(a) = &song.album {
            Album::get_or_create(&mut self.db, a, artist).ok().map(|mut a| {
                // Increment album length to include the new song
                a.length += 1;
                a.push_updates(&mut self.db).unwrap();
                a.id
            })
        } else { None };

        let song = match Song::create(&mut self.db, Some(&job_id), song.title.as_ref().unwrap(), artist, album, 123456) {
            Ok(s) => s,
            Err(e) => return Task::done(Message::DatabaseError(Arc::new(e)))
        };

        return Task::done(Message::SongInstalled(song));
    }

    pub(super) fn download_song(&self, song: Song) -> Task {
        iced::Task::future(crate::jobs::download::song::yt(song.ytid.as_ref().expect("tried to download a non-yt song").clone()))
            .map(move |r| Message::SongDownload(Arc::new(r.map(|_| song.clone()))))
    }

    pub(super) fn download_song_callback(&mut self, result: Arc<crate::Result<Song>>) -> Task {
        // TODO: check if song actually downloaded successfully
        let mut song = match &*result {
            Ok(song) => song.clone(),
            Err(e) => { println!("[dl] error downloading song: {e:?}"); return Task::none(); }
        };
        song.downloaded = true;
        if let Err(e) = song.push_updates(&mut self.db) { return Task::done(Message::DatabaseError(Arc::new(e))) };
        println!("[dl] {:?} downloaded successfully", song.name);
        Task::none()
    }

    pub(super) fn handle_song_installed(&mut self, song: Song) -> Task {
        println!("[met] installed \"{}\"", song.name);
        Task::none()
    }
}
