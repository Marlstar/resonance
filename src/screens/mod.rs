use crate::settings::Settings;

pub mod playing;
pub mod settings;

pub struct Screens {
    pub playing: playing::PlayingScreen,
    pub settings: settings::SettingsScreen,
}
impl Screens {
    pub fn create(settings: Settings) -> Self {
        Self {
            playing: playing::PlayingScreen::default(),
            settings: settings::SettingsScreen::new(settings)
        }
    }
}
