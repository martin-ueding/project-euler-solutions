use std::{
    collections::{HashMap, HashSet},
    fs,
};

use itertools::Itertools;

fn solution() -> i64 {
    let content = fs::read_to_string("../data/0098_words.txt").expect("File couldn't be found!");
    let row = content.trim();
    let json_string = format!("[{}]", row);
    let vec: Vec<String> = serde_json::from_str(&json_string).expect("Invalid JSON!");

    let mut anagram_classes: HashMap<String, Vec<String>> = HashMap::new();
    for s in vec {
        let class: String = s.chars().sorted().collect();
        anagram_classes.entry(class).or_default().push(s);
    }

    anagram_classes.retain(|_class, strings| strings.len() > 1);

    println!("{:?}", anagram_classes);
    0
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 98,
        implementations: &[("", solution)],
    }
}
