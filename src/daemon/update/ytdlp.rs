use youtube_dl::SingleVideo;
use std::sync::Arc;
use crate::models::{Album, Artist, Song};
use crate::iced::types::Task;
use crate::daemon::Message;

impl super::super::Daemon {
    pub(super) fn get_song_metadata(&mut self, ytid: String) -> Task {
        println!("[met] getting metadata for job {ytid}");
        return iced::Task::future(crate::jobs::metadata::song::yt(ytid.clone()))
            .map(move |r| Message::SongMetadata(ytid.clone(), Arc::new(r)));
    }

    pub(super) fn song_metadata_callback(&mut self, job_id: String, result: Arc<crate::Result<Box<SingleVideo>>>) -> Task {
        let song = match &*result {
            Ok(vid) => vid,
            Err(e) => {
                println!("[met] error getting metadata for job {job_id} ({e:?})");
                return Task::none();
            }
        };

        let artist = song.artist.as_ref()
            .map(|a| Artist::get_or_create(a))
            .and_then(Result::ok)
            .map(|a| a.id);

        let album = song.album.as_ref()
            .map(|a| Album::get_or_create(a, artist))
            .and_then(Result::ok)
            .map(|mut album| {
                album.length += 1;
                album.push_updates().unwrap();
                album.id
            });

        let song = match Song::create(Some(&job_id), song.title.as_ref().unwrap(), artist, album, 123456) {
            Ok(s) => s,
            Err(e) => return Task::done(Message::DatabaseError(Arc::new(e)))
        };

        return Task::done(Message::SongInstalled(song));
    }

    pub(super) fn download_song(&self, song: Song) -> Task {
        println!("[dl] downloading \"{}\"", song.name);
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
        if let Err(e) = song.push_updates() { return Task::done(Message::DatabaseError(Arc::new(e))) };
        println!("[dl] {:?} downloaded successfully", song.name);
        Task::none()
    }

    pub(super) fn handle_song_installed(&mut self, song: Song) -> Task {
        println!("[met] installed \"{}\"", song.name);
        Task::none()
    }
}
