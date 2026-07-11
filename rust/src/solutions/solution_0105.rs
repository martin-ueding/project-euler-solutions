use std::fs;

fn solution() -> i64 {
    let sets = load_data();
    0
}

fn load_data() -> Vec<Vec<i64>> {
    fs::read_to_string("../data/0105_sets.txt")
        .expect("File couldn't be found!")
        .lines()
        .map(line_to_vec)
        .collect()
}

fn line_to_vec(line: &str) -> Vec<i64> {
    line.split(',')
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect()
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 104,
        implementations: &[
            ("", solution)
        ]
    }
}
