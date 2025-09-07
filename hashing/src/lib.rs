pub fn mean(list: &[i32]) -> f64 {
    list.iter().sum::<i32>() as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted = list.to_vec();
    sorted.sort();
    let len = sorted.len();
    if len % 2 == 0 {
        (sorted[len / 2] + sorted[len / 2 - 1]) / 2
    } else {
        sorted[len / 2]
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut counts = std::collections::HashMap::new();
    for &num in list {
        *counts.entry(num).or_insert(0) += 1;
    }
    let mut max_count = 0;
    let mut mode = 0;
    for (&num, &count) in &counts {
        if count > max_count {
            max_count = count;
            mode = num;
        }
    }
    mode
}
