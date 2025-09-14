pub fn talking(s: &str) -> &str {
    match s {
        s if s.trim().is_empty() => "Just say something!",
        s if s.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase())
            && s.trim_end().ends_with('?') => "Quiet, I am thinking!",
        s if s.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase()) => {
            "There is no need to yell, calm down!"
        }
        s if s.trim_end().ends_with('?') => "Sure.",
        _ => "Interesting",
    }
}
