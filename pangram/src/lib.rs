pub fn is_pangram(s: &str) -> bool {
    let mut chars: Vec<char> = s.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect();
    chars.sort();
    chars.dedup();
    chars.len() == 26
}
