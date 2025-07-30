use tray_item::TrayItem;
use tray_item::IconSource;
use std::sync::OnceLock;
use async_channel::{unbounded, Sender, Receiver};

mod subscription;
pub use subscription::subscription;

static EVENT_RX: OnceLock<Receiver<TrayEvent>> = OnceLock::new();

pub fn create() -> TrayItem {
    let (tx, rx) = unbounded::<TrayEvent>();
    let _ = EVENT_RX.get_or_init(move || rx);

    let icon = IconSource::Data {
        width: 256, height: 256,
        data: crate::assets::icon_rgba_256().to_vec(),
    };
    let mut tray = TrayItem::new("Resonance", icon).expect("failed to initialise tray icon");
    
    tray.add_menu_item("Open", sender(tx.clone(), TrayEvent::Open)).unwrap();
    tray.add_menu_item("Settings", sender(tx.clone(), TrayEvent::Settings)).unwrap();
    tray.add_menu_item("Exit", sender(tx.clone(), TrayEvent::Exit)).unwrap();

    return tray;
}

fn sender(tx: Sender<TrayEvent>, msg: TrayEvent) -> impl Fn() {
    move || { tx.send_blocking(msg.clone()).unwrap(); }
}

#[derive(Debug, Clone)]
pub enum TrayEvent {
    Open,
    Settings,
    Exit,
}
