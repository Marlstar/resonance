use std::path::Path;
use youtube_dl::YoutubeDl;
use super::COVER_FORMAT;

pub async fn yt(ytid: String) -> crate::Result<()> {
    let path = &*crate::dirs::SONGS; // Base path, not the specific song
    let url = crate::util::yt_url_from_ytid(&ytid);
    let mut ytdlp = crate::deps::ytdlp::new(url);

    ytdlp_args(&mut ytdlp, &format!("y-{ytid}"));

    ytdlp.download_to_async(path).await?;

    convert_thumbnail(&crate::dirs::cover::yt_intermediate(&ytid))?;

    return Ok(());
}

pub fn convert_thumbnail(path: &Path) -> crate::Result<()> {
    let filename = path.file_stem().unwrap().to_string_lossy();
    let out_path = path.parent().unwrap().join(format!("{filename}.{COVER_FORMAT}"));

    let img = image::open(path)?;
    let size = img.height();
    let padding = (img.width() - size)/2;
    let cropped = img.crop_imm(padding, 0, size, size);
    cropped.save(out_path)?;

    // Get rid of the webp
    std::fs::remove_file(path)?;

    return Ok(());
}

fn ytdlp_args(ytdlp: &mut YoutubeDl, filename: &str) {
    ytdlp.output_template(format!("{filename}.%(ext)s"));
    ytdlp.extract_audio(true);
    ytdlp.extra_arg("--audio-format");
    ytdlp.extra_arg(super::AUDIO_FORMAT);
    ytdlp.extra_arg("--write-thumbnail");
}
