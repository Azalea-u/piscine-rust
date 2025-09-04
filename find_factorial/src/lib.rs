pub fn factorial(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}
