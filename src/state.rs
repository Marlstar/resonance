use serde::{Serialize, Deserialize};
use hashbrown::HashSet;
use crate::Song;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct State {
    pub songs: HashSet<Song>
}
impl State {
    pub fn new() -> Self {
        Default::default()
    }
}
