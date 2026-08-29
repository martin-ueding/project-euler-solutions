use crate::digits::{first_9_digits_pandigital_log_mantissa, last_9_digits_pandigital};
use crate::fibonacci::{FibonacciSuffixIterator, approximate_fibonacci_log10};
use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};

fn solution_modulus() -> i64 {
    let bar = ProgressBar::no_length()
        .with_style(ProgressStyle::with_template("{elapsed_precise} {pos} ({per_sec})").unwrap());
    FibonacciSuffixIterator::new()
        .progress_with(bar)
        .enumerate()
        .filter(|(_, f)| last_9_digits_pandigital(*f))
        .filter(|(i, _f)| {
            first_9_digits_pandigital_log_mantissa(approximate_fibonacci_log10(*i as i64 + 1))
        })
        .next()
        .unwrap()
        .0 as i64
        + 1
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 104,
        implementations: &[
            ("modulus", solution_modulus),
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
