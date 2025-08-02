use super::cover::convert_thumbnail;

pub async fn yt(ytid: String) -> crate::Result<()> {
    let path = &*crate::dirs::SONGS; // Base path, not the specific song
    let cover_path = &*crate::dirs::cover::yt(&ytid);
    let cover_downloaded = cover_path.exists();
    let url = crate::util::yt_url_from_ytid(&ytid);
    let mut ytdlp = crate::deps::ytdlp::new(url);

    ytdlp.output_template(format!("y-{ytid}.%(ext)s"));
    ytdlp.extract_audio(true);
    ytdlp.extra_arg("--audio-format");
    ytdlp.extra_arg(super::AUDIO_FORMAT);

    if !cover_downloaded { ytdlp.extra_arg("--write-thumbnail"); }

    ytdlp.download_to_async(path).await?;

    if !cover_downloaded { convert_thumbnail(&crate::dirs::cover::yt_intermediate(&ytid))?; }

    return Ok(());
}
