use youtube_dl::{SingleVideo, YoutubeDl};
use std::path::PathBuf;
use crate::Error;

impl super::Resonance {
    pub(super) fn download_song(&mut self, url: &str) -> Result<(SingleVideo, PathBuf), Error> {
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
        let out_path = out_dir.join("song.m4a");

        ytdl.output_template("song.%(ext)s");
        ytdl.extract_audio(true);
        ytdl.format("140");
        ytdl.download_to(&out_dir)?;

        return Ok((info, out_path));
    }
}

mod tests {
    #[test]
    fn yt_regex() {
        assert_eq!("j37GED-AR3M", crate::util::get_ytid_from_url("https://music.youtube.com/watch?v=j37GED-AR3M&si=neasnr4EEfdzDD_P").unwrap());
        assert_eq!("algfSDGHAG8advuav", crate::util::get_ytid_from_url("https://music.youtube.com/watch?v=algfSDGHAG8advuav&si=neasnr4EEfdzDD_P").unwrap());
    }
}
