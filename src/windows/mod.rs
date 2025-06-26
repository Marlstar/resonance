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
