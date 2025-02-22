#![allow(clippy::needless_return)]

mod player;
pub use player::Player;

mod input;
pub use input::get_input;

mod dirs;
pub use dirs::dirs;

mod song;
pub use song::Song;

mod state;
pub use state::State;
