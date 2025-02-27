use crate::prompt_input;
use crate::State;
use crate::state::StateError;

mod download;
use download::DownloadError;

pub struct Player {
    state: State
}
impl Player {
    pub fn new() -> Result<Self, StateError> {
        return Ok(Self {
            state: State::init()?
        });
    }
}
impl Player {
    pub fn run(&mut self) {
        let url = prompt_input("Enter song URL to download");
        let _ = self.download(url.as_str());

        println!("Finished running, saving state to file");
        println!("Exiting!");
    }

    pub fn exit(self) {
        let _ = self.state.db.close();
    }

    pub fn download(&mut self, url: &str) -> Result<(), DownloadError> {
        let id = match crate::util::get_id_from_url(url) {
            Some(a) => a,
            None => return Err(DownloadError::InvalidURL)
        };

        // TODO: refactor to do this as part of the download function
        if self.state.check_song_by_id(&id) {
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

    pub fn delete(&mut self, id: &String) -> Result<(), StateError> {
        return self.state.remove_song(id);
    }

    pub fn rename(&mut self, id: &str, name: String) -> Result<(), StateError> {
        todo!();
    }
}
