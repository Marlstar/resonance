#![allow(clippy::needless_return)]

pub mod db;

mod error;
pub use error::Error;

mod resonance;
pub use resonance::Resonance;

mod input;
pub use input::get_input;
pub use input::prompt_input;

mod dirs;
pub use dirs::dirs;

mod models;
pub use models::{Song, NewSong};
pub use models::Playlist;

mod database;
pub use database::Database;

pub mod util;

mod cli;
pub use cli::CLI;
