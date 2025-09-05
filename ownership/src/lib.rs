pub fn first_subword( s: String) -> String {
    let mut chars = s.chars();
    let mut f_subword = String::new();
    while let Some(c) = chars.next() {
        if (c == '_' || c.is_uppercase()) && !f_subword.is_empty() {
            break;
        }
        f_subword.push(c);
    }
    f_subword
}
