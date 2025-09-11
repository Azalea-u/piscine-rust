
pub struct Message {
    pub content: String,
    pub sender: String,
}

impl Message {
    pub fn new(content: String, sender: String) -> Self {
        Message { content, sender }
    }
}

pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.contains("stupid") || message.is_empty() {
        Err("ERROR: illegal")
    } else {
        Ok(message)
    }
}
