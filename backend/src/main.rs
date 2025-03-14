fn main() {
    //let mut db = resonance::Database::load().unwrap();
    let cli = resonance_backend::CLI::new().unwrap();
    drop(cli.run());
}
