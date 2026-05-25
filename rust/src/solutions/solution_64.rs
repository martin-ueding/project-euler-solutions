use crate::fractions::expand_root;

fn solution() -> i64 {
    (2..10_000)
        .filter(|&number| expand_root(number).1.len() % 2 == 1)
        .count() as i64
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 64,
        implementations: &[("", solution)],
        solution: Some(1322),
    }
}
