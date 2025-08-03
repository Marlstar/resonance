use crate::iced::types::Task;
use crate::settings::Settings;
use crate::daemon::Message;

pub fn save_to_file() -> Task {
    iced::Task::future(Settings::save())
        .map(|_| Message::None)
}

pub fn update_global(new: Settings) -> Task {
    iced::Task::future(Settings::set(new))
        .map(|_| Message::None)
}
