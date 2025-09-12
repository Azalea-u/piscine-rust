#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let encoded: String = original.chars().map(|c| atbash(c)).collect();

    match encoded == ciphered {
        true => Ok(()),
        false => Err(CipherError { expected: encoded }),
    }
}

fn atbash(c: char) -> char {
    match c {
        'A'..='Z' => (b'Z' - (c as u8 - b'A')) as char,
        'a'..='z' => (b'z' - (c as u8 - b'a')) as char,
        _ => c,
    }
}
