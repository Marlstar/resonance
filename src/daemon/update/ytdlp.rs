use youtube_dl::SingleVideo;
use std::sync::Arc;
use crate::models::{Artist, Song};
use crate::iced::types::Task;
use crate::daemon::Message;

impl super::super::Daemon {
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

        match Song::create(&mut self.db, Some(&job_id), song.title.as_ref().unwrap(), artist, None, 123456) {
            Ok(_) => Task::none(),
            Err(e) => Task::done(Message::InsertFailed(Arc::new(e)))
        }
    }
}
