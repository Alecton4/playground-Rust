use std::collections::HashMap;

pub fn demo_hash_map_basics() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();

    // Add entries to the hashmap
    // Note that we are not passing references
    // So the ownership of the strings will be moved into the hashmap
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    // Get value from hashmap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // Iterate over all elements
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

pub fn demo_hash_map_update() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20); // overwrite

    scores.entry(String::from("Yellow")).or_insert(30); // create "Yellow" entry with value 30
    scores.entry(String::from("Yellow")).or_insert(40); // will not overwrite
}

pub fn demo_hash_map_update_2() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let mut count = map.entry(word).or_insert(0);
        // ??? why the following is wrong
        // let mut count = map.entry(&word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
