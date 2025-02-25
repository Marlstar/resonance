use std::path::{Path, PathBuf};
use ytdlp_bindings::{YtDlp, YtDlpError};
use regex::Regex;
use crate::Song;
use super::Player;

impl super::Player {
    pub(super) fn download_song(&mut self, url: &str) -> Result<Song, DownloadError> {
        let id = Player::get_id_from_url(url)?;
        let path = Self::get_path(id.as_str());
        Self::download_audio(url, path)?;
        return Ok(Song{
            url: url.to_string(),
            file: Self::get_path(id.as_str()),
            id,
            name: None
        })
    }

    fn download_audio(url: &str, path: impl AsRef<Path>) -> Result<(), YtDlpError> {
        YtDlp::new()?.download_audio(url, path)
    }

    pub(super) fn get_id_from_url(url: &str) -> Result<String, DownloadError> {
        // https://regex101.com/r/dgnOi5/3
        let re = Regex::new(r".*\.youtube\.com/watch\?v=(?<id>(?:\w|-)+).*").unwrap();
        let captures = match re.captures(url) {
            Some(a) => a,
            None => return Err(DownloadError::IDNotFound)
        };
        return Ok(captures["id"].to_string());
    }

    fn get_path(id: &str) -> PathBuf {
        return crate::dirs().audio_file(id);
    }
}

#[derive(Debug)]
pub enum DownloadError {
    InvalidID,
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
        assert_eq!("j37GED-AR3M", crate::Player::get_id_from_url("https://music.youtube.com/watch?v=j37GED-AR3M&si=neasnr4EEfdzDD_P").unwrap());
        assert_eq!("algfSDGHAG8advuav", crate::Player::get_id_from_url("https://music.youtube.com/watch?v=algfSDGHAG8advuav&si=neasnr4EEfdzDD_P").unwrap());
    }

    #[test]
    fn path() {
        assert_eq!(crate::Player::get_path("music"), crate::dirs().audio_files().join("music.mp3"));
    }
}
