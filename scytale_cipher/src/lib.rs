pub fn scytale_cipher(message: &str, rows: usize) -> String {
    if rows == 0 || message.is_empty() {
        return String::new();
    }

    let chars: Vec<char> = message.chars().collect();
    let cols = (chars.len() + rows - 1) / rows;
    let mut grid = vec![' '; rows * cols];
    grid[..chars.len()].copy_from_slice(&chars);

    let mut result = String::with_capacity(rows * cols);
    for r in 0..rows {
        for c in 0..cols {
            result.push(grid[c * rows + r]);
        }
    }
    result.trim_end().to_string()
}
