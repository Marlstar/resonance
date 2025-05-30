use youtube_dl::YoutubeDl;
use essi_ffmpeg::FFmpeg;
use essi_ffmpeg::FFmpegDownloadProgress;

use crate::util::flush_stdout;

struct ProgressBar {
    printed: usize,
}
impl ProgressBar {
    pub fn new() -> Self {
        Self { printed: 0 }
    }

    pub fn start(&self) {
        print!(" > ");
        flush_stdout();
    }

    pub fn update(&mut self, percentage: usize) {
        let a = percentage / 10;
        if a > self.printed {
            for _ in 0..(a - self.printed) { print!("#") }
            flush_stdout();
            self.printed = a;
        }
    }
}

pub fn ffmpeg_path() -> std::path::PathBuf {
    crate::dirs().deps().join("ffmpeg").join("ffmpeg")
}
pub async fn install_ffmpeg() {
    FFmpeg::override_downloaded_ffmpeg_path(crate::dirs().deps()).unwrap();

    if let Some((_handle, mut progress)) = FFmpeg::auto_download().await.unwrap() {
        println!("[deps] ffmpeg not found, installing");

        let (mut started, mut extracting) = (false, false);
        let mut progressbar = ProgressBar::new();

        'recv: while let Some(state) = progress.recv().await {
            match state {
                FFmpegDownloadProgress::Starting => {},
                FFmpegDownloadProgress::Downloading(status) => {
                    if !started {
                        started = true;
                        print!("[install-ffmpeg] downloading");
                        flush_stdout();
                        progressbar.start();
                    }
                    if let Some(s) = status { progressbar.update(s); }
                },
                FFmpegDownloadProgress::Extracting => {
                    if !extracting {
                        println!(" => DONE");
                        extracting = true;
                        print!("[install-ffmpeg] extracting");
                        flush_stdout();
                    }
                },
                FFmpegDownloadProgress::Finished => {
                    println!(" => DONE");
                    println!("[install-ffmpeg] install complete");
                    break 'recv;
                },
            }
        }
    } else {
        println!("[deps] ffmpeg installation found");
    }
}
pub fn ffmpeg_installed() -> bool {
    return crate::dirs().deps().join("ffmpeg/ffmpeg").exists();
}

pub fn ytdlp(url: impl Into<String>) -> YoutubeDl {
    let mut ytdlp = YoutubeDl::new(url);
    ytdlp.youtube_dl_path(ytdlp_path());

    if FFmpeg::get_program().unwrap() != Some("ffmpeg".to_string()) {
        ytdlp.extra_arg("--ffmpeg-location");
        ytdlp.extra_arg(FFmpeg::get_program().unwrap().unwrap());
    }
        
    return ytdlp;
}
pub fn ytdlp_path() -> std::path::PathBuf {
    crate::dirs().deps().join("yt-dlp").join("yt-dlp")
}
pub async fn install_ytdlp() {
    use youtube_dl::downloader::download_yt_dlp;
    let path = crate::dirs().deps().join("yt-dlp");
    if path.exists() {
        println!("[deps] yt-dlp installation found")
    } else {
        match download_yt_dlp(path).await {
            Ok(_) => {
                println!("[install-yt-dlp] installation complete");
            },
            Err(e) => {
                println!("[install-yt-dlp] installation failed with error {e:?}");
                std::process::exit(1);
            }
        }
    }
}
pub fn ytdlp_installed() -> bool {
    return crate::dirs().deps().join("yt-dlp/yt-dlp").exists();
}

pub fn install_deps() {
    let f = ffmpeg_installed();
    let y = ytdlp_installed();
    if f && y { return; }

    let rt = tokio::runtime::Runtime::new().unwrap();
    if !f { rt.block_on(install_ffmpeg()); }
    if !y { rt.block_on(install_ytdlp()); }
}
