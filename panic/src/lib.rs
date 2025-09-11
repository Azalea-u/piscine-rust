use std::fs::File;
use std::io::ErrorKind;

pub fn open_file(s: &str) -> File {
    match File::open(s) {
        Ok(file) => file,
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                panic!("No such file or directory: {}", s);
            } else {
                panic!("Failed to open file: {}", error);
            }
        }
    }
}
