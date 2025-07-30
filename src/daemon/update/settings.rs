use crate::iced::types::Task;
use crate::settings::Settings;

impl super::super::Daemon {
    pub(super) fn settings_update(&mut self, settings: Settings) -> Task {
        println!("[settings] applied changes");
        self.settings = settings.clone();
        self.screens.settings.update(settings);
        self.settings.save();
        return Task::none();
    }
}
