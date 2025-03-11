pub mod home;
pub use home::Home;

#[derive(Debug, Clone)]
pub enum Screen {
    Home(Home),
}
impl Screen {
    pub fn view<'a>(&self) -> iced::Element<'a, crate::Message> {
        match self {
            Screen::Home(s) => s.view(),
        }
    }
}

pub trait ScreenCore {
    type Message;
    fn view<'a>(&self) -> iced::Element<'a, crate::Message>;
    fn handle_message(&mut self, msg: Self::Message) -> crate::Task;
}
