use std::path::PathBuf;
use essi_ffmpeg::FFmpeg;
use essi_ffmpeg::FFmpegDownloadProgress;

pub async fn install() {
    FFmpeg::override_downloaded_ffmpeg_path(crate::dirs::DEPENDENCIES.clone()).unwrap();

    if let Some((_handle, mut progress)) = FFmpeg::auto_download().await.unwrap() {
        println!("[deps/ffmpeg] not found, installing");

        let mut extracting = false;

        while let Some(state) = progress.recv().await {
            match state {
                FFmpegDownloadProgress::Starting => println!("[deps/ffmpeg] downloading"),
                FFmpegDownloadProgress::Downloading(_status) => {},
                FFmpegDownloadProgress::Extracting => {
                    if !extracting {
                        println!("[deps/ffmpeg] extracting");
                        extracting = true;
                    }
                },
                FFmpegDownloadProgress::Finished => println!("[deps/ffmpeg] installed successfully"),
            }
        }
    } else {
        let install_type = match path().to_str().unwrap() {
            "ffmpeg" => "system",
            _ => "local",
        };
        println!("[deps/ffmpeg] installation found ({install_type})");
    }
}
pub fn is_installed() -> bool {
    return path().exists();
}
pub fn path() -> PathBuf {
    return FFmpeg::get_program().unwrap().unwrap().into();
}
