fn main() {
    println!("You ran the backend silly");
    // tokio::runtime::Runtime::new().unwrap().block_on(resonance_backend::deps::install_ffmpeg());
    tokio::runtime::Runtime::new().unwrap().block_on(resonance_backend::deps::install_ytdlp());
}
