pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut initials = Vec::new();
    for name in names {
        let mut initial = String::new();
        for word in name.split_whitespace() {
            initial.push( word.chars().next().unwrap());
            initial.push('.');
            initial.push(' ');
        }
        initial.pop();
        initials.push(initial);
    }
    initials
}
