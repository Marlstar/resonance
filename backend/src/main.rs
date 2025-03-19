fn main() {
    let mut r = resonance_backend::Resonance::new().unwrap();
    let song = r.get_song(2).unwrap();
    r.audio.play_song(song);
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("skipping");
    let song = r.get_song(4).unwrap();
    r.audio.play_song(song);
    std::thread::sleep(std::time::Duration::from_secs(10));
}
