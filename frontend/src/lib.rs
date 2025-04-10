#![allow(clippy::needless_return)]

pub type Task = iced::Task<Message>;

mod resonance;
pub use resonance::Resonance;

mod message;
pub use message::Message;

pub mod screens;

pub mod tasks;

pub mod appearance;

pub mod widgets;

pub mod assets;
