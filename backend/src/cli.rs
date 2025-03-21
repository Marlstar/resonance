use crate::{get_input, prompt_input, Error};
use colored::Colorize;

type Args = Vec<String>;

pub struct CLI {
    resonance: crate::Resonance
}
impl CLI {
    pub fn new() -> Result<Self, Error> {
        return Ok(Self {
            resonance: crate::Resonance::new()?
        });
    }
}
impl CLI {
    pub async fn run(mut self) {
        'main: loop {
            let (cmd, args) = Self::get_command();
            match cmd.as_str() {
                "help" => self.help(),
                "download" => self.download(args).await,
                "search" => self.search(),
                "rename" => self.rename(),
                "delete" => self.delete(args),
                "list" => self.list(),
                "exit" => break 'main,
                "" => (),
                unknown => println!("Unknown command \"{unknown}\"")
            }

            println!(" "); // Leave a gap between commands
        }

        self.resonance.exit();
    }
}
impl CLI { // Managing input
    fn get_command() -> (String, Args) {
        let input = get_input();
        let mut iter = input.split(" ");
        let command = iter.next().unwrap_or("").to_string();
        let args: Args = iter.map(|a| a.to_string()).collect();
        return (command, args);
    }
}
impl CLI { // Commands
    fn help(&self) {
        let cmd = |c: &str, d: &str| {
            let count = 10 - c.chars().count();
            let spacing = (0..count).map(|_| " ").collect::<Vec<&str>>().join("");
            println!("{}:{spacing}{d}", c.purple());
        };
        println!("Available commands:");
        cmd("download", "Download a song by URL");
        cmd("search", "Search YouTube Music");
        cmd("rename", "Rename a song by ID");
        cmd("list", "List all downloaded songs");
        cmd("help", "Show this help page");
        cmd("exit", "Quit Resonance CLI");
    }

    async fn download(&mut self, args: Args) {
        let url = if args.is_empty() {
            prompt_input("Enter URL to download")
        } else { args[0].clone() };

        let vid = crate::download_song(&url).await.unwrap();
        let result = self.resonance.install_downloaded(vid);

        match result {
            Ok(song) => {
                let purple = |text: &str| text.purple();
                println!("Downloaded \"{}\" by {} ({}s)", purple(&song.name), purple(&song.author), purple(&format!("{}", &song.duration)));
            },
            Err(e) => { println!("Error: {e:?}") }
        }
    }

    fn search(&mut self) {
        let query = prompt_input("Query");
        let _ = self.resonance.search(&query, 1);
    }

    fn rename(&mut self) {
        let id = prompt_input("Enter song YTID");
        let name = prompt_input("New name");
        let _result = self.resonance.rename_by_ytid(&id, &name);
    }

    fn delete(&mut self, args: Args) {
        let id = if args.is_empty() {
            prompt_input("Song id")
        } else { args[0].clone() }.parse::<i32>().unwrap();

        let _result = self.resonance.delete(id);
    }

    fn list(&mut self) {
        let _ = self.resonance.list_songs();
    } 
}
