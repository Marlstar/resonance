use std::path::Path;
use regex::Regex;

use crate::Error;

pub fn display_path(p: &Path) -> String {
    return format!("{}", p.display());
}

pub fn ytid_from_yt_url(url: &str) -> Result<String, Error> {
    let re = Regex::new(r".*\.youtube\.com/watch\?v=(?<id>(?:\w|-)+).*")?;
    let captures = match re.captures(url) {
        Some(a) => a,
        None => return Err(Error::InvalidYTURL),
    };
    return Ok(captures["id"].to_string());
}

pub fn yt_url_from_ytid(id: &str) -> String {
    return format!("https://music.youtube.com/watch?v={id}");
}

pub fn millis_to_formatted_duration(millis: i32) -> String {
    let seconds = millis / 1000;
    let minutes = seconds / 60;
    let seconds = seconds - (minutes * 60);
    format!("{minutes}:{seconds}")
}

mod tests {
    #[test]
    pub fn ytid_from_yt_url() {
        assert_eq!(
            super::ytid_from_yt_url("https://music.youtube.com/watch?v=ZqVDbGlDzzo&si=pBYLnOQnrJ5-o7xV").ok(),
            Some("ZqVDbGlDzzo".to_string())
        );
    }

    #[test]
    pub fn millis_to_duration() {
        assert_eq!(
            super::millis_to_formatted_duration(78300),
            "1:18"
        );
    }
}
