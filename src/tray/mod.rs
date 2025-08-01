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

    let icon = crate::assets::icon_rgba_256().to_vec();
    let mut bytes = Vec::<u8>::with_capacity(icon.len());
    let mut iter = icon.into_iter();
    while let Some(r) = iter.next() {
        let g = iter.next().unwrap();
        let b = iter.next().unwrap();
        let a = iter.next().unwrap();
        // convert to ARGB
        bytes.push(a);
        bytes.push(r);
        bytes.push(g);
        bytes.push(b);
    };

    let icon = IconSource::Data {
        width: 256, height: 256,
        data: bytes,
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
