use crate::audio::handler::AudioHandler;
use crate::settings::Settings;
use crate::windows::Windows;
use crate::screens::{Screen, Screens};
use crate::tasks;

mod update;
mod view;
mod subscriptions;
mod boot;
mod exit;

mod message;
pub use message::Message;

pub struct Daemon {
    pub audio: AudioHandler,
    pub tray: tray_item::TrayItem,
    
    pub windows: Windows,
    pub screens: Screens,

    pub current_screen: Screen,

    pub ffmpeg_ready: bool,
    pub ytdlp_ready: bool,

    pub settings: Settings,
}
impl Daemon {
    pub fn new() -> Self {
        let audio = AudioHandler::new().expect("failed to initialise audio handler");
        let settings = Settings::load_or_default();

        let tray = crate::tray::create();

        let windows = Windows::default();
        let screens = Screens::create(settings.clone());

        return Self {
            audio,
            tray,
            windows,
            screens,
            current_screen: Screen::Library,
            ffmpeg_ready: false,
            ytdlp_ready: false,
            settings,
        };
    }

    pub fn window_titles(&self, id: iced::window::Id) -> String {
        self.windows.get_title(id)
    }
}
impl Default for Daemon {
    fn default() -> Self { Self::new() }
}
