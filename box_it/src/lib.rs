pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    s.split_whitespace()
        .filter_map(|num_str| {
            let n = if let Some(num_str) = num_str.strip_suffix('k') {
                num_str.parse::<f64>().ok().map(|n| (n * 1000.0) as u32)
            } else {
                num_str.parse::<f64>().ok().map(|n| n as u32)
            };
            n.map(Box::new)
        })
        .collect()
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    a.into_iter().map(|i| *i).collect()
}