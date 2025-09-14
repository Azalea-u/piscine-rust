pub fn  number_logic(num: i32) -> bool {
    let mut tmp = num;
    let mut sum = 0;
    let mut degits = Vec::new();

    while tmp > 0 {
        degits.push(tmp % 10);
        tmp /= 10;
    }
    for i in &degits {
        sum += i.pow(degits.len() as u32);
    }
    sum == num
}
