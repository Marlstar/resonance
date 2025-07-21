use std::collections::HashMap;
use iced::window::Id;

pub mod main;

pub struct Windows {
    pub main: Option<Id>,
    pub popups: HashMap<String, Id>,
}
impl Default for Windows {
    fn default() -> Self {
        Self {
            main: None,
            popups: HashMap::new(),
        }
    }
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
        closed!(main, id)
    }
}
