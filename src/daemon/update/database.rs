use std::sync::Arc;
use crate::iced::types::Task;

impl super::super::Daemon {
    pub(super) fn handle_database_error(&self, e: Arc<diesel::result::Error>) -> Task {
        println!("[db] database error: {e:?}");
        Task::none()
    }
}
