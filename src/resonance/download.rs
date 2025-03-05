use std::path::{Path, PathBuf};
use ytdlp_bindings::{YtDlp, YtDlpError};
use crate::Error;

impl super::Resonance {
    pub(super) fn download_song(&mut self, url: &str) -> Result<PathBuf, Error> {
        let id = match crate::util::get_ytid_from_url(url) {
            Some(a) => a,
            None => return Err(Error::InvalidURL),
        };
        let path = crate::dirs().audio_file(&id);
        Self::download_audio(url, &path)?;
        return Ok(path);
    }

    fn download_audio(url: &str, path: impl AsRef<Path>) -> Result<(), YtDlpError> {
        YtDlp::new()?.download_audio(url, path)
    }
}

mod tests {
    #[test]
    fn yt_regex() {
        assert_eq!("j37GED-AR3M", crate::util::get_ytid_from_url("https://music.youtube.com/watch?v=j37GED-AR3M&si=neasnr4EEfdzDD_P").unwrap());
        assert_eq!("algfSDGHAG8advuav", crate::util::get_ytid_from_url("https://music.youtube.com/watch?v=algfSDGHAG8advuav&si=neasnr4EEfdzDD_P").unwrap());
    }
}
