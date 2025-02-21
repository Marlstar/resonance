use std::path::Path;
use ytdlp_bindings::{YtDlp, YtDlpError};

pub fn download(url: &str, path: impl AsRef<Path>) -> Result<(), YtDlpError> {
    YtDlp::new()?.download_audio(url, path)
}
