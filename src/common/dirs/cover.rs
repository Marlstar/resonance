use std::path::PathBuf;
use crate::tasks::download::COVER_FORMAT;

pub fn yt(ytid: &str) -> PathBuf {
    return crate::dirs::SONGS.join(yt_filename(ytid));
}
pub fn yt_filename(ytid: &str) -> String {
    format!("y-{ytid}.{COVER_FORMAT}")
}
pub fn yt_intermediate(ytid: &str) -> PathBuf {
    return crate::dirs::SONGS.join(yt_filename_intermediate(ytid));
}
pub fn yt_filename_intermediate(ytid: &str) -> String {
    format!("y-{ytid}.webp")
}
