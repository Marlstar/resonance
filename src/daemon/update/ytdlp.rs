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

        match Song::create(&mut self.db, Some(&job_id), song.title.as_ref().unwrap(), artist, album, 123456) {
            // TODO: remove auto download
            Ok(s) => Task::done(s.download()),
            Err(e) => Task::done(Message::InsertFailed(Arc::new(e)))
        }
    }

    pub(super) fn download_song(&self, song: Song) -> Task {
        iced::Task::future(crate::jobs::download::song::yt(song.ytid.as_ref().expect("tried to download a non-yt song").clone()))
            .map(move |r| Message::SongDownload(song.clone(), Arc::new(r)))
    }

    pub(super) fn download_song_callback(&mut self, mut song: Song, result: Arc<crate::Result<()>>) -> Task {
        // TODO: check if song actually downloaded successfully
        song.downloaded = true;
        song.push_updates(&mut self.db);
        println!("[dl] \"{:?}\" by \"{:?}\" downloaded successfully", song.name, song.artist);
        Task::none()
    }
}
