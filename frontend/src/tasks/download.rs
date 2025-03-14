use crate::Message;
use backend::download_song;

pub async fn download(url: String) -> Message {
    match download_song(&url).await {
        Ok(song) => Message::DownloadComplete(url.to_string(), song),
        Err(_) => Message::DownloadFailed(url.to_string())
    }
}
