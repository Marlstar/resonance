use std::path::PathBuf;

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
        return self.dirs.data_dir().to_owned().join("player");
    }

    pub fn audio_files(&self) -> PathBuf {
        return self.base().join("audio")
    }
}
