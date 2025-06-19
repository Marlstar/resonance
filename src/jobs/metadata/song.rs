use youtube_dl::{SingleVideo, YoutubeDlOutput};
use crate::Error;

pub async fn yt(ytid: &str) -> crate::Result<Box<SingleVideo>> {
    let url = crate::util::yt_url_from_ytid(ytid);
    let ytdlp = crate::deps::ytdlp::new(url);

    let result = ytdlp.run_async().await?;
    let song = match result {
        YoutubeDlOutput::SingleVideo(s) => s,
        YoutubeDlOutput::Playlist(_) => return Err(Error::InvalidYTID), // Shouldn't be possible but we have to match it anyway
    };
    return Ok(song);
}
