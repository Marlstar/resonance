pub mod song;
pub use song::Song;

pub mod album;
pub use album::Album;

pub mod artist;
pub use artist::Artist;

mod get_model;
pub use get_model::*;

pub mod prelude {
    pub use super::Song;
    pub use super::Artist;
    pub use super::Album;

    pub use super::get_model::*;
}
