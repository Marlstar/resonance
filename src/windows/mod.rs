use std::collections::HashMap;
use iced::window::Id;

pub mod main;

#[derive(Default)]
pub struct Windows {
    pub main: Option<Id>,
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
        closed!(main, id)
    }
}
