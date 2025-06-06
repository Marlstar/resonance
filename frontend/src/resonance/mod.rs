use crate::screens::Screen;
use crate::screens::Library;

mod update;
mod view;
mod subscription;

pub struct Resonance {
    backend: backend::Resonance,
    screen: Screen,
    theme: iced::Theme,
}
impl Default for Resonance {
    fn default() -> Self {
        Self::new()
    }
}
impl Resonance {
    pub fn new() -> Resonance {
        let mut backend = Self::try_load_backend();
        let songs = backend.list_songs().unwrap();

        return Self {
            backend,
            screen: Screen::Library(Library::new(songs)),
            theme: iced::Theme::CatppuccinMocha,
        };
    }
    
    fn try_load_backend() -> backend::Resonance {
        return match backend::Resonance::new() {
            Ok(a) => a,
            Err(e) => {
                eprintln!("Error initialising backend: {e:?}");
                std::process::exit(1);
            }
        };
    }

    pub fn theme(&self) -> iced::Theme {
        self.theme.clone()
    }

    pub fn window_settings() -> iced::window::Settings {
        iced::window::Settings {
            icon: Some(iced::window::icon::from_rgba(crate::assets::icon_rgba_256().to_vec(), 256, 256).unwrap()),
            ..Default::default()
        }
    }
}
