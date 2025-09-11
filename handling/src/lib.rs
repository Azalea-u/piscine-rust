use std::path::Path;
use std::fs::File;
use std::io::Write;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = match File::options().append(true).create(true).open(path) {
        Ok(file) => file,
        Err(error) => panic!("Failed to open or create file: {}", error),
    };
    if let Err(error) = file.write_all(content.as_bytes()) {
        panic!("Failed to write to file: {}", error);
    }
}


pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.contains("stupid") || message.is_empty() {
        Err("ERROR: illegal")
    } else {
        Ok(message)
    }
}
