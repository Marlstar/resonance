fn main() {
    //let mut db = resonance::Database::load().unwrap();
    let cli = resonance::CLI::new().unwrap();
    cli.run();
}
