use std::path::Path;

pub async fn load_song_bytes(path: impl AsRef<Path>) -> crate::Result<Vec<u8>> {
    return Ok(tokio::fs::read(path).await?);
}
