use std::path::{Path, PathBuf};
use ytdlp_bindings::{YtDlp, YtDlpError};
use crate::Song;

impl super::Player {
    pub(super) fn download_song(&mut self, url: &str) -> Result<Song, DownloadError> {
        let id = match crate::util::get_id_from_url(url) {
            Some(a) => a,
            None => return Err(DownloadError::InvalidURL),
        };
        let path = Self::get_path(id.as_str());
        Self::download_audio(url, path)?;
        return Ok(Song{
            url: url.to_string(),
            path: Self::get_path_string(&id),
            // TODO: song name
            name: String::from("song name"),
            // TODO: song duration
            duration: 2,
            id,
        })
    }

    fn download_audio(url: &str, path: impl AsRef<Path>) -> Result<(), YtDlpError> {
        YtDlp::new()?.download_audio(url, path)
    }

    fn get_path(id: &str) -> PathBuf {
        return crate::dirs().audio_file(id);
    }
    fn get_path_string(id: &str) -> String {
        let path = Self::get_path(id);
        let s = format!("{path:?}");
        let trimmed = s.trim_start_matches("\"").trim_end_matches("\"");
        return trimmed.to_string();
    }
}

#[derive(Debug)]
pub enum DownloadError {
    InvalidURL,
    IDNotFound,
    YtDlp(YtDlpError),
    State(crate::state::StateError)
}
impl From<YtDlpError> for DownloadError {
    fn from(value: YtDlpError) -> Self {
        return DownloadError::YtDlp(value);
    }
}
impl From<crate::state::StateError> for DownloadError {
    fn from(value: crate::state::StateError) -> Self {
        return DownloadError::State(value);
    }
}

mod tests {
    #[test]
    fn yt_regex() {
        assert_eq!("j37GED-AR3M", crate::util::get_id_from_url("https://music.youtube.com/watch?v=j37GED-AR3M&si=neasnr4EEfdzDD_P").unwrap());
        assert_eq!("algfSDGHAG8advuav", crate::util::get_id_from_url("https://music.youtube.com/watch?v=algfSDGHAG8advuav&si=neasnr4EEfdzDD_P").unwrap());
    }

    #[test]
    fn path() {
        assert_eq!(crate::Player::get_path("music"), crate::dirs().audio_files().join("music.mp3"));
    }
}
