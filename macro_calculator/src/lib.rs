use json::{JsonValue, object};

pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

fn round_and_trim(value: f64) -> JsonValue {
    let rounded = (value * 100.0).round() / 100.0;
    let s = format!("{:.2}", rounded);
    
    JsonValue::from(s.trim_end_matches('0').trim_end_matches('.').parse::<f64>().unwrap())
}

pub fn calculate_macros(foods: &[Food]) -> JsonValue {
    let mut cals = 0.0;
    let mut carbs = 0.0;
    let mut proteins = 0.0;
    let mut fats = 0.0;

    for food in foods {
        let kcal_str = food.calories.1.trim_end_matches("kcal");
        let kcal = kcal_str.parse::<f64>().unwrap_or(0.0);

        cals += kcal * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
    }

    object! {
        "cals" => round_and_trim(cals),
        "carbs" => round_and_trim(carbs),
        "proteins" => round_and_trim(proteins),
        "fats" => round_and_trim(fats)
    }
}
