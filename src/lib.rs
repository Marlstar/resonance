#![allow(clippy::needless_return, clippy::new_without_default)]

pub mod daemon;

pub mod tray;

pub mod iced;

mod error;
pub use error::Error;
pub use error::Result;

pub mod common;
pub use common::dirs;

pub mod tasks;

pub mod db;
pub mod models;

pub mod audio;

pub mod deps;
pub mod util;
pub mod assets;
