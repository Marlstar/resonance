use crate::screens::Home;
use crate::screens::Library;
use crate::screens::LibraryMessage;
use crate::screens::Playing;
use crate::screens::PlayingMessage;
use crate::screens::ScreenCore;
use crate::Message;
use crate::tasks;
use crate::Task;
use crate::screens::Screen;
use backend::SingleVideo;
use backend::QueueEvent;
use colored::Colorize;

impl super::Resonance {
    pub fn update(&mut self, message: Message) -> Task {
        return match message {
            Message::None => Task::none(),

            Message::Mpris(received) => self.handle_mpris_message(received),
            Message::Seek(pos) => self.seek(pos),
            Message::SeekRelative(offset) => self.seek_relative(offset),
            Message::SeekUpdate => self.seek_update(),

            Message::Download(url) => self.download(url),
            Message::DownloadComplete(url, vid) => self.download_complete(&url, vid),
            Message::DownloadFailed(url) => self.download_failed(&url),

            Message::DeleteSong(id) => self.delete_song(id),

            Message::PlaySong(id) => self.play_song(id),
            Message::Queue(event) => self.queue_event(event),
            Message::Skip(num) => self.skip(num),
            Message::PauseSong => self.pause_song(),
            Message::ResumeSong => self.resume_song(),

            Message::SwitchToHomeScreen => self.switch_to_home(),
            Message::SwitchToLibraryScreen => self.switch_to_library(),
            Message::SwitchToPlayingScreen => self.switch_to_playing(),

            Message::Home(msg) => {
                if let Screen::Home(home) = &mut self.screen {
                    home.handle_message(msg)
                }
                else { Task::none() }
            },

            Message::Library(msg) => {
                if let Screen::Library(lib) = &mut self.screen {
                    match msg {
                        LibraryMessage::Refresh => self.refresh_library(),
                        _ => lib.handle_message(msg)
                    }
                } else { Task::none() }
            },

            Message::Playing(msg) => {
                if let Screen::Playing(screen) = &mut self.screen {
                    screen.handle_message(msg)
                }
                else { Task::none() }
            },
        }
    }
}
impl super::Resonance {
    fn switch_to_home(&mut self) -> Task {
        self.screen = Screen::Home(Home::new());
        Task::none()
    }

    fn switch_to_library(&mut self) -> Task {
        let lib = Library::new(self.backend.list_songs().unwrap());
        self.screen = Screen::Library(lib);
        Task::none()
    }

    fn switch_to_playing(&mut self) -> Task {
        if let Some(song) = self.backend.audio.current() {
            self.screen = Screen::Playing(Playing::new(song.clone()));
            return Task::done(Message::Playing(crate::screens::PlayingMessage::Update(song.clone())));
        }
        Task::none()
    }
}
impl super::Resonance {
    fn handle_mpris_message(&mut self, message: backend::mpris::Recv) -> Task {
        use backend::mpris::Recv;
        Task::done(match message {
            Recv::Play => Message::ResumeSong,
            Recv::Pause => Message::PauseSong,
            Recv::PlayPause => if self.backend.audio.playing { Message::PauseSong } else { Message::ResumeSong },
            Recv::Position(t) => Message::Seek(t.as_secs() as f32),
            Recv::SeekRelative(t) => Message::SeekRelative(t.as_secs() as f32),
        })
    }

    fn seek(&mut self, pos: f32) -> Task {
        self.backend.audio.seek(pos);
        Task::none()
    }

    fn seek_relative(&mut self, offset: f32) -> Task {
        self.backend.audio.seek_relative(offset);
        Task::none()
    }

    fn seek_update(&mut self) -> Task {
        self.backend.audio.seek_update();
        Task::done(Message::Playing(PlayingMessage::Seek(self.backend.audio.position)))
    }

    fn download(&mut self, url: String) -> Task {
        Task::future(tasks::download(url))
    }

    fn download_complete(&mut self, _url: &str, vid: SingleVideo) -> Task {
        // TODO: error handling
        println!("Downloaded {} by {}", vid.title.clone().unwrap().purple(), vid.channel.clone().unwrap().purple());
        let _ = self.backend.install_downloaded(vid);
        Task::done(Message::Library(LibraryMessage::Refresh))
    }

    fn download_failed(&self, url: &str) -> Task {
        // TODO: handle failed downloads
        println!("Failed downloading {}", url.purple());
        Task::none()
    }

    fn delete_song(&mut self, id: i32) -> Task {
        if self.backend.delete(id).is_err() {
            eprintln!("failed to delete id {id}");
        };
        Task::none()
    }

    fn play_song(&mut self, id: i32) -> Task {
        // TODO: error handling
        let song = self.backend.get_song(id).unwrap();
        println!("Playing {} by {}", song.name, song.author);
        self.backend.audio.play_song(song);
        // TODO: don't auto-switch once things are implemented fully
        Task::done(Message::SwitchToPlayingScreen)
    }

    fn queue_event(&mut self, event: QueueEvent) -> Task {
        match event {
            QueueEvent::AddToEnd(song) => self.backend.audio.queue_add_back(song),
            e => todo!("queue event {e:?}"),
        }
        if !self.backend.audio.playing {
            // TODO: un-jankify
            Task::batch([
                Task::done(Message::Skip(1)),
                Task::done(Message::ResumeSong),
            ])
        } else { Task::none() }
    }

    fn skip(&mut self, num: i32) -> Task {
        for _ in 0..num.abs() {
            self.backend.audio.skip(num > 0);
        }
        Task::done(Message::Playing(PlayingMessage::Update(self.backend.audio.current_song.clone().unwrap())))
    }

    fn pause_song(&mut self) -> Task {
        self.backend.audio.pause();
        Task::done(Message::Playing(PlayingMessage::PlayingStatus(false)))
    }

    fn resume_song(&mut self) -> Task {
        self.backend.audio.resume();
        Task::done(Message::Playing(PlayingMessage::PlayingStatus(true)))
    }

    fn refresh_library(&mut self) -> Task {
        let songs = self.backend.list_songs().unwrap();
        Task::done(Message::Library(LibraryMessage::RefreshResponse(songs)))
    }
}
