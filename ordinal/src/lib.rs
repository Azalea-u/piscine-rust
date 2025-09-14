pub fn num_to_ordinal(x: u32) -> String {
    match x {
        11 | 12 | 13 => format!("{}th", x),
        x => match x%10 {
            1 => format!("{}st", x),
            2 => format!("{}nd", x),
            3 => format!("{}rd", x),
            _ => format!("{}th", x),
        }
    }
}
