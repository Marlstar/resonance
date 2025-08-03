use async_lock::RwLock;
use std::sync::LazyLock;
use super::Settings;

pub static SETTINGS: LazyLock<RwLock<Settings>> = LazyLock::new(|| RwLock::new(Settings::load_or_default()));

impl super::Settings {
    pub async fn get() -> Settings {
        SETTINGS.read().await.clone()
    }

    pub fn get_blocking() -> Settings {
        SETTINGS.read_blocking().clone()
    }

    pub async fn set(new: Settings) {
        *SETTINGS.write().await = new;
    }

    pub fn set_blocking(new: Settings) {
        *SETTINGS.write_blocking() = new;
    }

    pub async fn save() {
        let settings = Self::get().await;
        Self::save_to_file(&settings);
    }

    pub fn save_blocking() {
        let settings = Self::get_blocking();
        Self::save_to_file(&settings);
    }
}
