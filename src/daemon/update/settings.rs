use crate::iced::types::Task;
use crate::settings::Settings;
use crate::tasks;

impl super::super::Daemon {
    pub(super) fn settings_update(&mut self, settings: Settings) -> Task {
        println!("[settings] applying changes");

        self.screens.settings.update(settings.clone());

        return tasks::settings::update_global(settings)
            .chain(tasks::settings::save_to_file())
    }
}
