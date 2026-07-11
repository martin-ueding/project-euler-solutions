use indicatif::ProgressIterator;
use num_traits::ToPrimitive;

use crate::digits::{first_9_digits_pandigital, last_9_digits_pandigital};
use crate::fibonacci::BigFibonacciIterator;

fn solution_big_iterator() -> i64 {
    BigFibonacciIterator::new()
        .progress_count(1_000_000_000)
        .enumerate()
        .filter(|(_, f)| last_9_digits_pandigital((f % 1_000_000_000i64).to_i64().unwrap()))
        .filter(|(_, f)| first_9_digits_pandigital(f.clone()))
        .next()
        .unwrap()
        .0 as i64
        + 1
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 104,
        implementations: &[("big iterator", solution_big_iterator)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fibonacci::FibonacciSuffixIterator;

    #[test]
    fn f_541_is_pandigital() {
        let fs: Vec<i64> = FibonacciSuffixIterator::new().take(542).collect();
        let f_541 = fs[540];
        println!("{f_541}");
        assert_eq!(f_541 % 1_000_000_000, 839_725_641);
        assert!(last_9_digits_pandigital(f_541));
    }

    #[test]
    fn find_541() {
        let (i, f) = FibonacciSuffixIterator::new()
            .enumerate()
            .filter(|(_, f)| last_9_digits_pandigital(*f))
            .next()
            .unwrap();
        println!("{f}");
        assert_eq!(i + 1, 541);
    }
}
