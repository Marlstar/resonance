#![allow(clippy::needless_return)]

pub mod db;

mod error;
pub use error::Error;

mod resonance;
pub use resonance::Resonance;

mod download;
pub use download::download_song;

mod blur;
pub use blur::blur;

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
pub use util::AM;

mod audio;
pub use audio::AudioPlayer;
pub use audio::QueueEvent;

pub mod mpris;

// Re-exports
pub use youtube_dl::SingleVideo;
pub use orx_linked_list as linked_list;
