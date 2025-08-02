use super::COVER_FORMAT;
use std::path::Path;

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

pub async fn yt(ytid: String) -> crate::Result<()> {
    let path = &*crate::dirs::SONGS; // Base path, not the specific song
    let cover_path = &*crate::dirs::cover::yt(&ytid);
    let cover_downloaded = cover_path.exists();
    let url = crate::util::yt_url_from_ytid(&ytid);
    let mut ytdlp = crate::deps::ytdlp::new(url);

    ytdlp.output_template(format!("y-{ytid}.%(ext)s"));
    ytdlp.extra_arg("--skip-download");
    ytdlp.extra_arg("--write-thumbnail");

    ytdlp.download_to_async(path).await?;

    if !cover_downloaded { convert_thumbnail(&crate::dirs::cover::yt_intermediate(&ytid))?; }

    return Ok(());
}
