use std::path::Path;
use regex::Regex;

use crate::Error;

pub fn display_path(p: &Path) -> String {
    return format!("{}", p.display());
}

pub fn ytid_from_yt_url(url: &str) -> Result<String, Error> {
    let re = Regex::new(r".*\.youtube\.com/watch\?v=(?<id>(?:\w|-)+).*")?;
    let captures = re.captures(url)?;
    return Ok(captures["id"].to_string());
}

pub fn yt_url_from_ytid(id: &str) -> String {
    return format!("https://music.youtube.com/watch?v={id}");
}

mod tests {
    #[test]
    pub fn ytid_from_yt_url() {
        assert_eq!(
            super::ytid_from_yt_url("https://music.youtube.com/watch?v=ZqVDbGlDzzo&si=pBYLnOQnrJ5-o7xV"),
            Some("ZqVDbGlDzzo".to_string())
        );
    }
}
