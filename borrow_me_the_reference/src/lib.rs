pub fn delete_and_backspace(s: &mut String) {
    let mut result = Vec::new();
    let mut skip_count = 0;

    for c in s.chars() {
    if skip_count > 0 && c != '-' && c != '+' {
            skip_count -= 1;

            continue;
        }

        match c {
            '-' => { result.pop(); }
            '+' => { skip_count += 1; }
            _ => { result.push(c); }
        }
    }

    *s = result.iter().collect();
}

pub fn do_operations(s: &mut [String]) {
    for elem in s {
        let mut split = elem.split(|c| c == '+' || c == '-');
        let num1 = split.next().unwrap().parse::<i32>().unwrap();
        let num2 = split.next().unwrap().parse::<i32>().unwrap();
        let operation = elem.chars().find(|c| *c == '+' || *c == '-').unwrap();

        match operation {
            '+' => *elem = (num1 + num2).to_string(),
            '-' => *elem = (num1 - num2).to_string(),
            _ => {}
        }

    
    }
}
