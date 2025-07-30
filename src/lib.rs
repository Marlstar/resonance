#![allow(clippy::needless_return, clippy::new_without_default)]

pub mod daemon;
pub mod windows;
pub mod screens;
pub mod tasks;
pub mod iced;

pub mod tray;

mod error;
pub use error::Error;
pub use error::Result;

pub mod common;
pub use common::dirs;

pub mod jobs;

pub mod db;
pub mod models;

pub mod audio;

pub mod deps;
pub mod util;
pub mod assets;
pub mod fonts;
pub mod settings;
