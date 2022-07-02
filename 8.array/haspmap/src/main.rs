use std::collections::HashMap;

fn main() {
    let mut v: HashMap<i32, i32> = HashMap::new();

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let intial_scores = vec![10, 50];

    let so: HashMap<_, _> = teams.iter().zip(intial_scores.iter()).collect();
}