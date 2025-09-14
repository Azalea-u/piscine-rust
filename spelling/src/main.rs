pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let below_20 = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen",
        "sixteen", "seventeen", "eighteen", "nineteen",
    ];

    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty",
        "sixty", "seventy", "eighty", "ninety",
    ];

    fn spell_hundreds(num: u64, below_20: &[&str], tens: &[&str]) -> String {
        let hundred = num / 100;
        let remainder = num % 100;
        let mut parts = Vec::new();

        if hundred > 0 {
            parts.push(format!("{} hundred", below_20[hundred as usize]));
        }

        if remainder > 0 {
            if remainder < 20 {
                parts.push(below_20[remainder as usize].to_string());
            } else {
                let ten_val = remainder / 10;
                let one_val = remainder % 10;
                let mut s = tens[ten_val as usize].to_string();
                if one_val > 0 {
                    s.push('-');
                    s.push_str(below_20[one_val as usize]);
                }
                parts.push(s);
            }
        }

        parts.join(" ")
    }

    const SCALE: [&str; 5] = ["", "thousand", "million", "billion", "trillion"];

    let mut num = n;
    let mut words = Vec::new();
    let mut scale_index = 0;

    while num > 0 {
        let chunk = num % 1000;
        if chunk > 0 {
            let mut part = spell_hundreds(chunk, &below_20, &tens);
            if !SCALE[scale_index].is_empty() {
                part.push(' ');
                part.push_str(SCALE[scale_index]);
            }
            words.insert(0, part);
        }
        num /= 1000;
        scale_index += 1;
    }

    words.join(" ")
}
