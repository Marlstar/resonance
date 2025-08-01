use crate::iced::types::Task;
use crate::screens::Screen;

impl super::super::Daemon {
    pub(super) fn focus_screen(&mut self, screen: Screen) -> Task {
        self.current_screen = screen;
        notify_focus(screen);
        match self.current_screen {
            Screen::Library => { println!("TODO: update library on open"); Task::none() }
            _ => Task::none()
        }
    }
}

fn notify_focus(screen: Screen) {
    println!("[screen] focused {screen:?}")
}
