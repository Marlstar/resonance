#![allow(clippy::needless_return)]

mod player;
pub use player::Player;

mod download;
pub use download::download;

mod input;
pub use input::get_input;

mod dirs;
pub use dirs::Dirs;

mod song;
pub use song::Song;

mod state;
pub use state::State;
