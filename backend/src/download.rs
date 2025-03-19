use youtube_dl::{SingleVideo, YoutubeDl};
use crate::Error;

pub async fn download_song(url: &str) -> Result<SingleVideo, Error> {
    let mut ytdl = YoutubeDl::new(url);
    ytdl.format("ba");

    // TODO: songs/playlists both in this function
    // `ytdl_output` is an enum of playlist and single video
    // see https://docs.rs/youtube_dl/0.10.0/youtube_dl/enum.YoutubeDlOutput.html
    let info = match ytdl.run()?.into_single_video() {
        Some(a) => a,
        None => return Err(Error::YtDlNotSingleVideo),
    };

    let out_dir = crate::dirs().song(&info.id);

    ytdl.output_template("song.%(ext)s");
    ytdl.extract_audio(true);
    ytdl.format("140");
    ytdl.extra_arg("--write-thumbnail");
    ytdl.download_to(&out_dir)?;

    // Crop thumbnail
    let uncropped_path = crate::dirs().song_thumbnail_uncropped(&info.id);
    let uncropped = image::open(&uncropped_path)?;
    let size = uncropped.height();
    let padding = (uncropped.width() - size)/2;
    let cropped = uncropped.crop_imm(padding, 0, size, size);
    cropped.save(crate::dirs().song_thumbnail(&info.id))?;

    let blurred = cropped.blur(15.0);
    blurred.save(crate::dirs().song_thumbnail_blurred(&info.id))?;

    // Remove uncropped image
    std::fs::remove_file(uncropped_path)?;

    return Ok(info);
}
