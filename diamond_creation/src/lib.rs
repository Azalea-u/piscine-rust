pub fn get_diamond(c: char) -> Vec<String> {
    let n = (c as u8 - b'A' + 1) as usize;
    let width = 2 * n - 1;
    let mut diamond: Vec<String> = Vec::with_capacity(width);

    for i in 0..n {
        let mut line = vec![' '; width];
        let letter = (b'A' + i as u8) as char;
        line[n - 1 - i] = letter;
        line[n - 1 + i] = letter;
        diamond.push(line.into_iter().collect());
    }

    for i in (0..n - 1).rev() {
        diamond.push(diamond[i].clone());
    }
    diamond
}
