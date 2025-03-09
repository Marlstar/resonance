use std::process::exit;
use colored::Colorize;

pub fn get_input() -> String {
    print!("> ");
    crate::util::flush_stdout();

    let mut buf = String::new();
    if std::io::stdin().read_line(&mut buf).inspect_err(|e| eprintln!("{}",format!("Error getting user input: {e}").red())).is_err() {
        exit(-1);
    }
    return String::from(buf.trim());
}

pub fn prompt_input(prompt: &str) -> String {
    println!("{prompt}");
    get_input()
}
