use crate::iced::types::Task;
use crate::daemon::Message;
use crate::models::Song;
use crate::jobs;
use crate::tray::TrayChangesExt;
use crate::models::ModelsExt;

impl super::super::Daemon {
    pub(super) fn load_song(&mut self, song: Song) -> Task {
        println!("[song] loading \"{}\"", song.name);
        iced::Task::future(jobs::io::load_song_bytes(song.path()))
            .map(Result::ok)
            .and_then(move |bytes| Message::LoadSongIntoSink(song.clone(), bytes).task())
    }

    pub(super) fn load_song_into_sink(&mut self, song: Song, bytes: Vec<u8>) -> Task {
        let name = song.name.clone();
        if let Err(e) = self.audio.load_song(song.clone(), bytes) {
            println!("[io] failed to load song \"{}\" ({e:?})", name);
        }

        return Task::batch([
            Message::Resume.task(),
            Message::UpdateSong(song).task(),
        ]);
    }

    pub(super) fn update_song(&mut self, song: Song) -> Task {
        self.screens.playing.update_song(Some(song.clone()));

        self.tray.change_song(&song.name);
        self.tray.change_artist(&song.artist.as_ref().unwrap().get_artist().unwrap().name);
        self.tray.change_album(&song.album.as_ref().unwrap().get_album().unwrap().name);

        Task::none()
    }
}
