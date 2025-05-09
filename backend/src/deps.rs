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

pub async fn install_ffmpeg() {
    use essi_ffmpeg::FFmpeg;
    use essi_ffmpeg::FFmpegDownloadProgress;

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
