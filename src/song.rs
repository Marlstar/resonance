use std::{hash::Hash, path::PathBuf};


#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Song {
    pub url: String,
    pub id: String,
    pub file: PathBuf,
}
impl Hash for Song {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.url.hash(state);
    }
}
