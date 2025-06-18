#![allow(clippy::needless_return)]

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
