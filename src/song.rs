use std::{hash::Hash, path::PathBuf};


#[derive(Debug, Clone, Eq, serde::Serialize, serde::Deserialize)]
pub struct Song {
    pub url: String,
    pub id: String,
    pub file: PathBuf,
}
impl Hash for Song {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl std::borrow::Borrow<String> for Song {
    fn borrow(&self) -> &String {
        &self.id
    }
}
impl PartialEq for Song {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
