pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap().to_string(),
        Security::Message => server.expect("ERROR: program stops").to_string(),
        Security::Warning => server.unwrap_or("WARNING: check the server").to_string(),
        Security::NotFound => match server {
            Ok(Url) => Url.to_string(),
            Err(Msg) => format!("Not found: {}", Msg),
        }
        Security::UnexpectedUrl => match server {
            Ok(Url) => panic!("{}", Url),
            Err(Msg) => format!("Not found: {}", Msg),
        }
    }
}
