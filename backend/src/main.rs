use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("You are running the BACKEND");
    let mut r = resonance_backend::Resonance::new().unwrap();
    for i in 1..=5 {
        let song = r.get_song(i).unwrap();
        r.audio.queue_add_back(song);
    }
    for _ in 1..=5 {
        r.audio.skip(true);
        sleep(Duration::from_secs(4));
    }
}
