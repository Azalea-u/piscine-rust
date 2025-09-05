pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut initials = Vec::new();
    for name in names {
        initials.push(name.chars().collect::<Vec<char>>()[0].to_string() + ".");
    }
    initials
}
