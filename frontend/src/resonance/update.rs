use crate::screens::ScreenCore;
use crate::Message;
use crate::tasks;
use crate::backend::SingleVideo;
use crate::Task;
use crate::screens::Screen;
use colored::Colorize;

impl super::Resonance {
    pub fn update(&mut self, message: Message) -> Task {
        return match message {
            Message::SwitchScreen(screen) => self.switch_screen(screen),

            Message::Download(url) => self.download(url),
            Message::DownloadComplete(url, vid) => self.download_complete(&url, vid),
            Message::DownloadFailed(url) => self.download_failed(&url),

            Message::DeleteSong(id) => self.delete_song(id),

            Message::Home(msg) => {
                if let Screen::Home(home) = &mut self.screen {
                    home.handle_message(msg)
                }
                else { Task::none() }
            }
        }
    }
}
impl super::Resonance {
    fn switch_screen(&mut self, screen: crate::screens::Screen) -> Task {
        self.screen = screen;
        Task::none()
    }

    fn download(&mut self, url: String) -> Task {
        Task::future(tasks::download(url))
    }

    fn download_complete(&mut self, _url: &str, vid: SingleVideo) -> Task {
        // TODO: error handling
        println!("Downloaded {} by {}", vid.title.clone().unwrap().purple(), vid.channel.clone().unwrap().purple());
        let _ = self.backend.install_downloaded(vid);
        Task::none()
    }

    fn download_failed(&self, url: &str) -> Task {
        // TODO: handle failed downloads
        println!("Failed downloading {}", url.purple());
        Task::none()
    }

    fn delete_song(&mut self, id: i32) -> Task {
        // TODO: error handling
        let _ = self.backend.delete(id);
        Task::none()
    }
}
