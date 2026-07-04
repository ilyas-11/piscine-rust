pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: &[Food]) -> json::JsonValue {
    let mut cals = 0.0;
    let mut carbs = 0.0;
    let mut proteins = 0.0;
    let mut fats = 0.0;

    for food in foods {
        cals += food.calories.1.replace("kcal", "").parse::<f64>().unwrap_or(0.0) * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
    }
    // println!("Total calories: {} kcal", cals);
    // println!("Total carbs: {} g", carbs);
    // println!("Total proteins: {} g", proteins);
    // println!("Total fats: {} g", fats);
    json::object! {
        "cals": round(cals),
        "carbs": round(carbs),
        "proteins": round(proteins),
        "fats": round(fats)
    }
}
fn round(n: f64) -> f64 {
    (n * 100.0).round() / 100.0
}
