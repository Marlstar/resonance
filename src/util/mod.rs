use std::path::Path;
use regex::Regex;

pub fn display_path(p: &Path) -> String {
    return format!("{}", p.display());
}

pub fn ytid_from_yt_url(url: &str) -> Option<String> {
    let re = Regex::new(r".*\.youtube\.com/watch\?v=(?<id>(?:\w|-)+).*").unwrap();
    let captures = re.captures(url)?;
    return Some(captures["id"].to_string());
}

pub fn yt_url_from_ytid(id: &str) -> String {
    return format!("https://music.youtube.com/watch?v={id}");
}
