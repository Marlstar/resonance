use crate::iced::types::Task;
use crate::screens::Screen;
use crate::daemon::Message;

impl super::super::Daemon {
    pub(super) fn focus_screen(&mut self, screen: Screen) -> Task {
        self.current_screen = screen;
        notify_focus(screen);
        match self.current_screen {
            Screen::Library => Message::UpdateLibrary.task(),
            _ => Task::none(),
        }
    }

    pub(super) fn update_library(&mut self) -> Task {
        self.screens.library.refresh_songs();
        Task::none()
    }
}

fn notify_focus(screen: Screen) {
    println!("[screen] focused {screen:?}")
}
