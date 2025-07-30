use tray_icon::menu::{Menu, MenuEvent, MenuId, MenuItem, MenuItemBuilder};
use tray_icon::{Icon, TrayIcon, TrayIconBuilder};
use async_channel::{Receiver, unbounded};
use std::sync::OnceLock;

static EVENT_RX: OnceLock<Receiver<MenuEvent>> = OnceLock::new();

mod subscription;
pub use subscription::subscription;

pub struct TrayHandler {
    _tray: Option<TrayIcon>,
}
impl TrayHandler {
    pub fn new() -> Self {
        let (tx, rx) = unbounded::<MenuEvent>();
        EVENT_RX.get_or_init(move || rx);
        MenuEvent::set_event_handler(Some(move |event| {
            tx.send_blocking(event).unwrap();
        }));

        #[cfg(target_os = "linux")]
        {
            std::thread::spawn(|| {
                gtk::init().unwrap();
                let menu = Self::build_menu();
                let _tray_icon = Self::tray_icon_builder(menu)
                    .build()
                    .expect("failed to create tray icon");
                gtk::main();
            });
            return Self { _tray: None };
        }

        #[cfg(not(target_os = "linux"))]
        {
            let menu = Self::build_menu();
            let tray = Self::tray_icon_builder(menu)
                .build()
                .expect("failed to create tray icon");

            return Self { _tray: Some(tray) };
        }
    }

    fn build_menu() -> Menu {
        fn item(title: &str, id: &str) -> MenuItem {
            MenuItemBuilder::new()
                .id(MenuId::new(id))
                .text(title)
                .enabled(true)
                .build()
        }

        // let menu = Menu::new();
        let menu = Menu::with_id_and_items(MenuId::new("main_menu"), &[
            &item("Open", "open"),
            &item("Settings", "settings"),
            &item("Exit", "exit"),
        ]).expect("failed to create menu");
        // TODO: error handling

        return menu;
    }

    fn tray_icon_builder(menu: Menu) -> TrayIconBuilder {
        return TrayIconBuilder::new()
            .with_title("Resonance")
            .with_icon(Icon::from_rgba(crate::assets::icon_rgba_256().clone(), 256, 256).unwrap())
            .with_menu(Box::new(menu))
    }
}
