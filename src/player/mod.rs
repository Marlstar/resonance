use crate::prompt_input;
use crate::State;
use crate::state::StateError;

mod download;
use download::DownloadError;

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
        let state = match State::load() {
            Ok(a) => {
                println!("Loaded state from file");
                a
            },
            Err(_) => {
                println!("No state loaded");
                State::default()
            }
        };
        return Self {
            state
        }
    }

    pub fn save_state(&self) -> Result<(), StateError> {
        self.state.save()
    }
}
impl Player {
    pub fn run(&mut self) {
        let url = prompt_input("Enter song URL to download");
        let _ = self.download(url.as_str());

        println!("Finished running, saving state to file");
        println!("Exiting!");
    }

    pub fn download(&mut self, url: &str) -> Result<(), DownloadError> {
        let id = Self::get_id_from_url(url)?;
        if self.state.song_is_installed(&id) {
            println!("Song ({id}) already installed!");
            return Err(DownloadError::State(StateError::SongAlreadyInstalled));
        }

        print!("Downloading song ({id})");
        crate::util::flush_stdout();

        let song = self.download_song(url)?;
        println!(" | DONE!");

        self.state.add_song(song)?;
        return Ok(());
    }
}
