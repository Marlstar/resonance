use tray_item::TrayItem;
use std::sync::OnceLock;

#[derive(Debug)]
pub struct TrayChangeableItems {
    pub song: u32,
    pub album: u32,
    pub artist: u32,
}

pub static CHANGEABLE_ITEMS: OnceLock<TrayChangeableItems> = OnceLock::new();

pub trait TrayChangesExt {
    fn change_song(&mut self, text: &str);
    fn change_album(&mut self, text: &str);
    fn change_artist(&mut self, text: &str);
}
impl TrayChangesExt for TrayItem {
    fn change_song(&mut self, text: &str) {
        self.inner_mut().set_menu_item_label(text, CHANGEABLE_ITEMS.get().unwrap().song).unwrap();
    }
    fn change_album(&mut self, text: &str) {
        self.inner_mut().set_menu_item_label(text, CHANGEABLE_ITEMS.get().unwrap().album).unwrap();
    }
    fn change_artist(&mut self, text: &str) {
        self.inner_mut().set_menu_item_label(text, CHANGEABLE_ITEMS.get().unwrap().artist).unwrap();
    }
}
