use std::collections::HashMap;
fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 50);
    scores.insert(String::from("Red"), 10);

    // Insert item if not exists
    scores.entry(String::from("Blue")).or_insert(90);

    //println!("{}", score.get("Blue").unwrap())

    //Iteration through HashMap
    for (key, value) in &scores {
        println!("{}, {}", key, value);
    }
}