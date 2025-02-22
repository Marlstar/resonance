use std::io::Write;

pub fn flush_stdout() {
    let _ = std::io::stdout().flush();
}
