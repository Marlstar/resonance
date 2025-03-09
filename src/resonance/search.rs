use youtube_dl::{SingleVideo, YoutubeDl};
use crate::Error;

const BASE_URL: &str = "https://music.youtube.com/search?q=";

pub fn search_youtube_for_ids(query: &str) -> Result<Vec<String>, Error> {
    let opts = format!("{BASE_URL}{query}");
    let mut ytdl = YoutubeDl::new(opts);
    ytdl.extra_arg("--skip-download");
    ytdl.extra_arg("--flat-playlist");
    let out = ytdl.run()?;
    let pl = match out.into_playlist() {
        Some(a) => a,
        None => return Err(Error::YtDlMalformedOutput)
    };
    let results = pl.entries.unwrap();
    return Ok(results.iter().map(|r| r.id.clone()).collect());
}

pub fn get_full_metadata(id: &str) -> Result<SingleVideo, Error> {
    return match YoutubeDl::new(format!("https://music.youtube.com/watch?v={id}"))
        .extra_arg("--skip-download")
        .run()?
        .into_single_video() {
            Some(a) => Ok(a),
            None => Err(Error::YtDlNotSingleVideo)
        };
}
