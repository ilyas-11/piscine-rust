pub fn check_ms(message: &str) -> Result<&str, &str> {
    match message {
        _ if message.contains("stupid")|| message.is_empty()=> Err("ERROR: illegal"),
        _ => Ok("hello there"),
    }
}