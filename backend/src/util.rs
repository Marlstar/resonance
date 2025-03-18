use std::io::Write;
use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, Mutex};

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


// Types
pub struct AM<T> {
    inner: Arc<Mutex<T>>
}
impl<T> Deref for AM<T> {
    type Target = Arc<Mutex<T>>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<T> AM<T> {
    pub fn new(inner: T) -> Self {
        return Self {
            inner: Arc::new(Mutex::new(inner))
        };
    }

    pub fn clone_am(&self) -> AM<T> {
        return Self {
            inner: Arc::clone(&self.inner)
        };
    }
}
