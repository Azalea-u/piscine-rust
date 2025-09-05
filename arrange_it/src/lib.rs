pub fn arrange_phrase(phrase: &str) -> String {
    let mut ordered: [Option<&str>; 9] = [None; 9];
    let mut count = 0;

    for word in phrase.split_whitespace() {
        let mut pos = 0;
        for c in word.chars() {
            if c.is_ascii_digit() {
                pos = c.to_digit(10).unwrap() as usize;
                break;
            }
        }
        if pos > 0 && pos <= 9 {
            ordered[pos - 1] = Some(word);
            count += 1;
        }
    }

    let mut result = String::with_capacity(phrase.len());

    let mut added = 0;
    for slot in ordered.iter().take(count) {
        if let Some(word) = slot {
            for c in word.chars() {
                if !c.is_ascii_digit() {
                    result.push(c);
                }
            }
            added += 1;
            if added < count {
                result.push(' ');
            }
        }
    }

    result
}
