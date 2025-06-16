use youtube_dl::YoutubeDl;
use super::ffmpeg;
use crate::util::display_path;

pub fn new(url: impl Into<String>) -> YoutubeDl {
    let mut ytdlp = YoutubeDl::new(url);
    ytdlp.youtube_dl_path(path());

    // TODO: make this an app setting
    ytdlp.extra_arg("--no-check-certificate");

    if ffmpeg::is_local_installation() {
        ytdlp.extra_arg("--ffmpeg-location");
        ytdlp.extra_arg(display_path(&ffmpeg::path()));
    }

    return ytdlp;
}

pub async fn install() {
    use youtube_dl::downloader::download_yt_dlp;

    if !path().exists() {
        println!("[deps/yt-dlp] not found, installing");
        match download_yt_dlp(&*crate::dirs::DEPENDENCIES).await {
            Ok(_) => println!("[deps/yt-dlp] installed successfully"),
            Err(e) => {
                println!("[deps/yt-dlp] installation failed with error {e:?}");
                std::process::exit(1);
            }
        }
    } else {
        println!("[deps/yt-dlp] installation found (local)");
    }
}
pub fn is_installed() -> bool {
    return path().exists();
}
pub fn path() -> std::path::PathBuf {
    return crate::dirs::DEPENDENCIES.join("yt-dlp");
}
