use crate::settings::Settings;

pub mod library;
pub mod playing;
pub mod settings;

pub struct Screens {
    pub library: library::LibraryScreen,
    pub playing: playing::PlayingScreen,
    pub settings: settings::SettingsScreen,
}
impl Screens {
    pub fn create(settings: Settings) -> Self {
        Self {
            library: library::LibraryScreen::default(),
            playing: playing::PlayingScreen::default(),
            settings: settings::SettingsScreen::new(settings)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Screen {
    Library,
    Playing,
    Settings,
}
