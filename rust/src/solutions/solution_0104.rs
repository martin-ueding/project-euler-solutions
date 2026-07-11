use crate::digits::last_9_digits_pandigital;
use crate::fibonacci::FibonacciSuffixIterator;

fn solution() -> i64 {
    0
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 103,
        implementations: &[("", solution)],
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
