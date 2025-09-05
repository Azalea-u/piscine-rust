pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(s: &str, pat: &str) -> bool {
    s.contains(pat)
}

pub fn split_at(s: &str, idx: usize) -> (&str, &str) {
    s.split_at(idx)
}

pub fn find(s: &str, pat: char) -> usize {
    s.find(pat).unwrap()
}
