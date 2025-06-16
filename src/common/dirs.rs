use std::sync::LazyLock;
use std::path::PathBuf;
use directories::BaseDirs;
pub static BASE: LazyLock<BaseDirs> = LazyLock::new(|| BaseDirs::new().unwrap());

macro_rules! path {
    ($name:ident, $ext:expr) => {
        pub static $name: LazyLock<PathBuf> = LazyLock::new(|| (&*BASE).state_dir().unwrap().join($ext).to_path_buf());
    };
    ($name:ident) => {
        pub static $name: LazyLock<PathBuf> = LazyLock::new(|| (&*BASE).state_dir().unwrap().to_path_buf());
    };
}

path!(DATABASE, "resonance.db");
