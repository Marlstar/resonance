use std::path::PathBuf;
use crate::jobs::download::AUDIO_FORMAT;

pub fn yt(ytid: &str) -> PathBuf {
    return crate::dirs::SONGS.join(yt_filename(ytid));
}
pub fn yt_filename(ytid: &str) -> String {
    format!("y-{ytid}.{AUDIO_FORMAT}")
}
