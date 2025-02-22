use crate::{get_input, prompt_input};
use colored::Colorize;

type Args = Vec<String>;

#[derive(Default)]
pub struct CLI {
    player: crate::Player
}
impl CLI {
    pub fn new() -> Self {
        Default::default()
    }
}
impl CLI {
    pub fn run(&mut self) {
        'main: loop {
            let (cmd, args) = Self::get_command();
            match cmd.as_str() {
                "help" => self.help(),
                "download" => self.download(args),
                "exit" => break 'main,
                "" => (),
                unknown => println!("Unknown command \"{unknown}\"")
            }

            println!(" "); // Leave a gap between commands
        }

        let _ = self.player.save_state();
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
        cmd("help", "Show this help page");
        cmd("exit", "Quit Player");
    }

    fn download(&mut self, args: Args) {
        let url = if args.is_empty() {
            prompt_input("Enter URL to download")
        } else { args[0].clone() };

        let _ = self.player.download(url.as_str());
    }
}
