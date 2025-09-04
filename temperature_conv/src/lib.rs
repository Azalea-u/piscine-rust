pub fn fahrenheit_to_celsius(f : f64) -> f64 {
    let c = (f - 32.0) / (9.0 / 5.0);
    return c;
}

pub fn celsius_to_fahrenheit(c : f64) -> f64 {
    let f = (c * 9.0 / 5.0) + 32.0;
    return f;
}