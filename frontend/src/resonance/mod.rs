use resonance_backend as backend;
use crate::screens::Screen;
use crate::screens::Home;

mod update;
mod view;

pub struct Resonance {
    backend: backend::Resonance,

    screen: Screen,
}
impl Default for Resonance {
    fn default() -> Self {
        Self::new()
    }
}
impl Resonance {
    pub fn new() -> Resonance {
        return Self {
            backend: Self::try_load_backend(),
            screen: Screen::Home(Home::new())
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
}
