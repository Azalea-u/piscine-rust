pub fn first_fifty_even_square() -> Vec<i32> {
    (1..).map(|n| n * 2).map(|n| n * n).take(50).collect()
}
