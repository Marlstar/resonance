use crate::iced::types::Task;
use crate::daemon::Message;
use crate::models::Song;
use crate::jobs;

impl super::super::Daemon {
    pub(super) fn load_song(&mut self, song: Song) -> Task {
        println!("[song] loading \"{}\"", song.name);
        iced::Task::future(jobs::io::load_song_bytes(song.path()))
            .map(Result::ok)
            .and_then(move |bytes| Task::done(Message::LoadSongIntoSink(song.clone(), bytes)))
    }

    pub(super) fn load_song_into_sink(&mut self, song: Song, bytes: Vec<u8>) -> Task {
        let name = song.name.clone();
        if let Err(e) = self.audio.load_song(song.clone(), bytes) {
            println!("[io] failed to load song \"{}\" ({e:?})", name);
        }

        self.screens.playing.update_song(Some(song));
        Task::none()
    }
}
