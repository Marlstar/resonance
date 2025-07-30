use std::collections::HashMap;
use iced::window::Id;

pub mod main;
pub mod settings;

#[derive(Default, Debug)]
pub struct Windows {
    pub main: Option<Id>,
    pub settings: Option<Id>,
    pub popups: HashMap<String, Id>,
}
impl Windows {
    pub fn update_closed(&mut self, id: Id) {
        macro_rules! closed {
            ($name:ident, $id:expr) => {
                if self.$name == Some($id) {
                    self.$name = None;
                    println!("[window] closed {}", stringify!($name));
                }
            }
        }
        closed!(main, id);
        closed!(settings, id);
    }

    pub fn get_title(&self, id: Id) -> String {
        macro_rules! title {
            ($window:ident, $title:expr, $id:expr) => {
                if self.$window == Some($id) { return String::from($title); }
            }
        }
        title!(main, "Main | Resonance", id);
        title!(settings, "Settings | Resonance", id);
        format!("Unknown window ({id}) | Resonance") // Default
    }
}
