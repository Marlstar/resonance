fn main() {
    let mut cli = player::CLI::new().unwrap();
    println!("created player");
    cli.run();
}
