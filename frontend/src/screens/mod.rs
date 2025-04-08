pub mod home;
pub use home::Home;
pub use home::HomeMessage;

pub mod library;
pub use library::Library;
pub use library::LibraryMessage;

pub mod playing;
pub use playing::Playing;
pub use playing::PlayingMessage;

#[derive(Debug, Clone)]
pub enum Screen {
    Home(Home),
    Library(Library),
    Playing(Playing),
}
impl Screen {
    pub fn view<'a>(&self, backend: &backend::Resonance) -> iced::Element<'a, crate::Message> {
        match self {
            Screen::Home(s) => s.view(backend),
            Screen::Library(s) => s.view(backend),
            Screen::Playing(s) => s.view(backend),
        }
    }
}

pub trait ScreenCore {
    type Message;
    fn view<'a>(&self, backend: &backend::Resonance) -> iced::Element<'a, crate::Message>;
    fn handle_message(&mut self, msg: Self::Message) -> crate::Task;
}
