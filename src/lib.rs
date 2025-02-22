#![allow(clippy::needless_return)]

mod player;
pub use player::Player;

mod input;
pub use input::get_input;
pub use input::prompt_input;

mod dirs;
pub use dirs::dirs;

mod song;
pub use song::Song;

mod state;
pub use state::State;

pub mod util;

mod cli;
pub use cli::CLI;
