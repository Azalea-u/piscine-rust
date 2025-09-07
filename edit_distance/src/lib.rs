pub fn edit_distance(source: &str, target: &str) -> usize {
    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();
    let mut table = vec![vec![0; target_chars.len() + 1]; source_chars.len() + 1];

    for i in 0..=source_chars.len() {
        table[i][0] = i;
    }
    for j in 0..=target_chars.len() {
        table[0][j] = j;
    }

    for i in 1..=source_chars.len() {
        for j in 1..=target_chars.len() {
            let substitution_cost = if source_chars[i - 1] == target_chars[j - 1] { 0 } else { 1 };
            table[i][j] = std::cmp::min(
                table[i - 1][j] + 1, // deletion
                std::cmp::min(table[i][j - 1] + 1, // insertion
                table[i - 1][j - 1] + substitution_cost), // substitution
            );
        }
    }

    table[source_chars.len()][target_chars.len()]
}
