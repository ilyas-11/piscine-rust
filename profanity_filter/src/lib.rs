

pub fn check_ms(message: &str) -> Result<&str, &str> {

  match message{
    message.contains("stupid")=>Err("ERROR: illegal"),
    _=>Ok("hello there"),
  }
}
