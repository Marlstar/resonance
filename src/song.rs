#[derive(Debug, Clone)]
pub struct Song {
    pub id: String,
    pub name: String,
    pub url: String,
    pub path: String,
    pub duration: usize,
}
