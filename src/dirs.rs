use std::path::PathBuf;
use std::sync::OnceLock;

pub static DIRS: OnceLock<Dirs> = OnceLock::new();
pub fn dirs() -> &'static Dirs {
    return DIRS.get_or_init(Dirs::new);
}

pub struct Dirs {
    dirs: directories::BaseDirs
}
impl Default for Dirs {
    fn default() -> Self {
        Self {
            dirs: directories::BaseDirs::new().unwrap()
        }
    }
}
impl Dirs {
    pub fn new() -> Self {
        Default::default()
    }
}
impl Dirs {
    pub fn base(&self) -> PathBuf {
        return self.dirs.data_dir().to_owned().join("resonance");
    }

    pub fn db(&self) -> PathBuf {
        return self.base().join("state.db");
    }
    pub fn db_backup(&self) -> PathBuf {
        return self.base().join("state.bak");
    }

    pub fn songs(&self) -> PathBuf {
        return self.base().join("songs");
    }
    /// Base directory for a song's files
    pub fn song(&self, ytid: &str) -> PathBuf {
        return self.songs().join(ytid);
    }
    /// Song audio file
    pub fn song_file(&self, ytid: &str) -> PathBuf {
        return self.song(ytid).join("song.m4a");
    }

    pub fn downloads(&self) -> PathBuf {
        return self.base().join("downloads");
    }
}
