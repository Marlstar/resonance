use crate::download;
use crate::get_input;
use crate::Dirs;
use crate::State;

pub struct Player {
    dirs: Dirs,
    state: State
}
impl Default for Player {
    fn default() -> Self {
        Player {
            dirs: Dirs::new(),
            state: State::new()
        }
    }
}
impl Player {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run(&self) {
        let url = get_input().unwrap();
        let url = url.trim();
        let _ = download(url, self.dirs.audio_files().join("audio.mp3"));
    }
}
