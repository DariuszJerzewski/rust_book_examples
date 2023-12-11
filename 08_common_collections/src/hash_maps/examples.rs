use std::collections::HashMap;

pub fn creating_and_modifying() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Team: {team_name}, score: {score}.");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let value = String::from("value");
    let key = String::from("key");
    let mut fav_colours = HashMap::new();
    fav_colours.insert(key, value); // key and value are moved here

    scores.insert(String::from("Blue"), 25); // overwritten Blue
    println!("{scores:?}");

    scores.entry(String::from("Green")).or_insert(50); // add if value not present
    scores.entry(String::from("Blue")).or_insert(50); // keep old value if present
    println!("{scores:?}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}