use std::fs;

use indicatif::ProgressIterator;

use crate::special_sum_sets::is_special_sum_set;

fn solution() -> i64 {
    let sets = load_data();
    sets.iter()
        .progress()
        .filter(|s| is_special_sum_set(*s))
        .map(|s| s.iter().sum::<i32>())
        .sum::<i32>() as i64
}

fn load_data() -> Vec<Vec<i32>> {
    fs::read_to_string("../data/0105_sets.txt")
        .expect("File couldn't be found!")
        .lines()
        .map(line_to_vec)
        .collect()
}

fn line_to_vec(line: &str) -> Vec<i32> {
    line.split(',')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect()
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 105,
        implementations: &[
            ("", solution)
        ]
    }
}
