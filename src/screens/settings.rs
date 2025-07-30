use iced::alignment::Vertical;
use iced::widget;
use iced::Fill;
use crate::daemon::Message;
use crate::settings::Settings;
use crate::iced::types::Element;

pub struct SettingsScreen {
    new: Settings,
    pub current: Settings,
}
impl SettingsScreen {
    pub fn new(settings: Settings) -> Self {
        Self {
            new: settings.clone(),
            current: settings,
        }
    }

    pub fn update(&mut self, updated: Settings) {
        self.current = updated.clone();
        self.new = updated;
    }

    pub fn handle_change(&mut self, setting: SettingChanged) {
        use SettingChanged::*;
        println!("[settings] update: {setting:?}");
        match setting {
            StartMinimised(s) => self.new.start_minimised = s,
        }
    }
}
impl SettingsScreen {
    pub fn view(&self) -> Element {
        let settings = widget::container(widget::column![
            setting_widgets::checkbox!("Start minimised", start_minimised, StartMinimised, self),
        ]).center(Fill);

        let mut apply = widget::button("Apply");
        if self.new != self.current { apply = apply.on_press_with(|| Message::SettingsUpdate(self.new.clone()))};

        return widget::column![
            settings,
            widget::row![apply],
        ].into();
    }
}


mod setting_widgets {
    #[macro_export]
    macro_rules! checkbox {
        ($title:expr, $setting:ident, $message:ident, $settings:expr) => {
            {
                let mut text = widget::text($title).size(16);
                if $settings.new.$setting != $settings.current.$setting { text = text.font(*$crate::fonts::BOLD); }

                let checkbox = widget::checkbox("", $settings.new.$setting)
                    .size(25)
                    .on_toggle(move |s| Message::SettingChanged(SettingChanged::$message(s)));

                let row = widget::row![checkbox, text]
                    .spacing(5)
                    .align_y(Vertical::Center);

                iced::Element::new(row)
            }
        }
    }
    pub use checkbox;
}

#[derive(Debug, Clone)]
pub enum SettingChanged {
    StartMinimised(bool)
}
