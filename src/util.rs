use std::io::Write;

pub fn flush_stdout() {
    let _ = std::io::stdout().flush();
}

pub fn get_id_from_url(url: &str) -> Option<String> {
    use regex::Regex;

    // https://regex101.com/r/dgnOi5/3
    let re = Regex::new(r".*\.youtube\.com/watch\?v=(?<id>(?:\w|-)+).*").unwrap();
    let captures = match re.captures(url) {
        Some(a) => a,
        None => return None
    };
    return Some(captures["id"].to_string());
}
