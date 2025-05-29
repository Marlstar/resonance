use crate::Message;
use backend::download_song;
use std::sync::Arc;

pub async fn download(url: String) -> Message {
    match download_song(&url).await {
        Ok(song) => Message::DownloadComplete(url.to_string(), song),
        Err(e) => Message::DownloadFailed(url.to_string(), Arc::new(e)),
    }
}
