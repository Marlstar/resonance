use std::sync::LazyLock;
use std::path::PathBuf;
use directories::BaseDirs;
pub static BASE: LazyLock<BaseDirs> = LazyLock::new(|| BaseDirs::new().unwrap());
pub static SHARE: LazyLock<PathBuf> = LazyLock::new(|| BASE.data_dir().join("resonancev2"));

pub mod song;
pub mod cover;

macro_rules! share {
    ($name:ident, $ext:expr) => {
        pub static $name: LazyLock<PathBuf> = LazyLock::new(|| (&*SHARE).join($ext).to_path_buf());
    };
    ($name:ident) => {
        pub static $name: LazyLock<PathBuf> = LazyLock::new(|| (&*SHARE).to_path_buf());
    };
}

share!(DATABASE, "resonance.db");
share!(DEPENDENCIES, "deps/");
share!(SONGS, "songs/");
