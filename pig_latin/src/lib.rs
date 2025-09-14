pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    text.split_whitespace()
        .map(|word| {
            let c: Vec<char> = word.chars().collect();
            if vowels.contains(&c[0]) {
                format!("{word}ay")
            } else if c.len() >= 3 && !vowels.contains(&c[0]) && c[1] == 'q' && c[2] == 'u' {
                format!("{}{}ay", &word[3..], &word[..3])
            } else {
                let i = c.iter().position(|ch| vowels.contains(ch)).unwrap_or(0);
                format!("{}{}ay", &word[i..], &word[..i])
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
