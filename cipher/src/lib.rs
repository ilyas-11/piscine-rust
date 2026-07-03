#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut expected = String::new();

    for c in original.chars() {
        let new_char = if c.is_ascii_lowercase() {
            (b'a' + b'z' - c as u8) as char
        } else if c.is_ascii_uppercase() {
            (b'A' + b'Z' - c as u8) as char
        } else {
            c
        };

        expected.push(new_char);
    }

    if expected == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected })
    }
}
