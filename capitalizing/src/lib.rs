pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + c.as_str(),
    }
}

pub fn title_case(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + c.as_str(),
    }
}

pub fn change_case(input: &str) -> String {
    input.chars().map(|c| {
        if c.is_lowercase() {
            c.to_uppercase().to_string()
        } else {
            c.to_lowercase().to_string()
        }
    }).collect()
}
