use crate::get_input;
use crate::State;

mod download;

pub struct Player {
    state: State
}
impl Default for Player {
    fn default() -> Self {
        Player {
            state: State::new()
        }
    }
}
impl Player {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run(&mut self) {
        let url = get_input().unwrap();
        let url = url.trim();
        //let _ = download(url, self.dirs.audio_files().join("audio.mp3"));
        let _ = self.download_song(url);
    }

    pub fn download(&mut self, url: &str) -> Result<(), download::DownloadError> {
        let song = self.download_song(url)?;
        self.state.songs.insert(song);
        return Ok(());
    }
}
