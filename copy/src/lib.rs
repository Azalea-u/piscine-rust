pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp = c as f64;
    let abs = c.abs() as f64;
    (c, exp.exp(), abs.ln())
}

pub fn str_function(a: String) -> (String, String) {
    let og = a.clone();
    let mut result = String::new();
    for c in a.split_whitespace() {
        let num = c.parse::<f64>().unwrap();
        result.push_str(&format!("{} ", num.exp()));
    }
    (og, result.trim().to_string())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let og = b.clone();
    let mut result = Vec::new();
    for c in b {
        result.push((c.abs() as f64).ln());
    }
    (og, result)
}
