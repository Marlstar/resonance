use youtube_dl::YoutubeDl;

pub async fn yt(ytid: &str) -> crate::Result<()> {
    let path = &*crate::dirs::SONGS; // Base path, not the specific song
    let url = crate::util::yt_url_from_ytid(ytid);
    let mut ytdlp = crate::deps::ytdlp::new(url);

    ytdlp_args(&mut ytdlp, &format!("y-{ytid}"));

    ytdlp.download_to_async(path).await?;

    return Ok(());
}

fn ytdlp_args(ytdlp: &mut YoutubeDl, filename: &str) {
    ytdlp.output_template(format!("{filename}.%(ext)s"));
    ytdlp.extract_audio(true);
    ytdlp.extra_arg("--audio-format");
    ytdlp.extra_arg(super::AUDIO_FORMAT);
}
