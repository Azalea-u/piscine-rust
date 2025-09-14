pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for i in array.iter().enumerate().rev() {
        if i.1 == &key {
            return Some(i.0);
        }
    }
    None
}
