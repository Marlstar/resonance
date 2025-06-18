use crate::daemon::Message;

pub type Task = iced::Task<Message>;
pub type Element<'a> = iced::Element<'a, Message>;
