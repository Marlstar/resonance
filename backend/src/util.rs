use std::io::Write;
use std::path::Path;

pub fn flush_stdout() {
    let _ = std::io::stdout().flush();
}

pub fn get_ytid_from_url(url: &str) -> Option<String> {
    use regex::Regex;

    // https://regex101.com/r/dgnOi5/3
    let re = Regex::new(r".*\.youtube\.com/watch\?v=(?<id>(?:\w|-)+).*").unwrap();
    let captures = re.captures(url)?;
    return Some(captures["id"].to_string());
}

pub fn path_to_string(path: &Path) -> String {
    format!("{}", path.display())
}
