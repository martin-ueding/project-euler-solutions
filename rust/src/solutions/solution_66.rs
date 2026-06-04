use indicatif::ProgressIterator;

use crate::{diophantine::DiophantineSolutionIterator, integers::is_square};

fn solution() -> i64 {
    let non_squares: Vec<i64> = (1..1001).filter(|&d| !is_square(d)).collect();

    non_squares
        .iter()
        .progress()
        .map(|&d| DiophantineSolutionIterator::new(d, 1).next().unwrap().0)
        .max()
        .unwrap()
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 66,
        implementations: &[("diophantine solutions", solution)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution(), 661);
    }
}
