pub fn stars(n: u32) -> String {
    let mut result = String::new();
    for _ in 0..2_u32.pow(n) {
        result += "*";
    }
    result
}
