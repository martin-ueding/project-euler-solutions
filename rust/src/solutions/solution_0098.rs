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

    let mut strings_by_length: HashMap<i64, HashSet<String>> = HashMap::new();
    for s in vec.iter() {
        strings_by_length
            .entry(s.len() as i64)
            .or_default()
            .insert(s.clone());
    }

    let mut anagram_classes: HashMap<String, Vec<String>> = HashMap::new();
    for s in vec.iter() {
        let class: String = s.chars().sorted().collect();
        anagram_classes.entry(class).or_default().push(s.clone());
    }

    anagram_classes.retain(|_class, strings| strings.len() > 1);

    println!("{:?}", anagram_classes);

    anagram_classes
        .iter()
        .map(|(_class, strings)| strings.len())
        .max()
        .unwrap() as i64
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 98,
        implementations: &[("", solution)],
    }
}
