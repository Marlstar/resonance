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
    pub fn run(mut self) {
        'main: loop {
            let (cmd, args) = Self::get_command();
            match cmd.as_str() {
                "help" => self.help(),
                "download" => self.download(args),
                "rename" => self.rename(),
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
        cmd("rename", "Rename a song by ID");
        cmd("list", "List all downloaded songs");
        cmd("help", "Show this help page");
        cmd("exit", "Quit Resonance CLI");
    }

    fn download(&mut self, args: Args) {
        let url = if args.is_empty() {
            prompt_input("Enter URL to download")
        } else { args[0].clone() };

        let result = self.resonance.download(url.as_str());
        match result {
            // TODO: nice output here
            Ok(song) => { dbg!(song); },
            Err(e) => { println!("Error: {e:?}") }
        }
    }

    fn rename(&mut self) {
        let id = prompt_input("Enter song YTID");
        let name = prompt_input("New name");
        let _result = self.resonance.rename_by_ytid(&id, &name);
    }

    fn list(&mut self) {
        let _ = self.resonance.list_songs();
    } 
}
