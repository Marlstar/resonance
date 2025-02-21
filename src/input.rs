pub fn get_input() -> Result<String, std::io::Error> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    return Ok(buf);
}
