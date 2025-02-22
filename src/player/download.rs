use std::path::{Path, PathBuf};
use ytdlp_bindings::{YtDlp, YtDlpError};
use regex::Regex;
use crate::Song;
use super::Player;

impl super::Player {
    pub(super) fn download_song(&mut self, url: &str) -> Result<Song, DownloadError> {
        let id = match Player::get_id_from_url(url) {
            Some(a) => a,
            None => return Err(DownloadError::InvalidID)
        };
        let path = Self::get_path(id.as_str());
        Self::download_audio(url, path)?;
        return Ok(Song{
            url: url.to_string(),
            file: Self::get_path(id.as_str()),
            id,
        })
    }

    fn download_audio(url: &str, path: impl AsRef<Path>) -> Result<(), YtDlpError> {
        YtDlp::new()?.download_audio(url, path)
    }

    fn get_id_from_url(url: &str) -> Option<String> {
        // https://regex101.com/r/dgnOi5/3
        let re = Regex::new(r".*\.youtube\.com/watch\?v=(?<id>(?:\w|-)+).*").unwrap();
        let captures = re.captures(url);
        return Some(captures?["id"].to_string());
    }

    fn get_path(id: &str) -> PathBuf {
        return crate::dirs().audio_file(id);
    }
}

pub enum DownloadError {
    InvalidID,
    YtDlp(YtDlpError)
}
impl From<YtDlpError> for DownloadError {
    fn from(value: YtDlpError) -> Self {
        return DownloadError::YtDlp(value);
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
