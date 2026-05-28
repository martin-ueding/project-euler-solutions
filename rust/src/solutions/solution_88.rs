use indicatif::ProgressIterator;
use std::collections::HashSet;

fn possible_products(n: i64, k: i64) -> HashSet<i64> {
    if k == 1 {
        HashSet::from([n])
    } else {
        let mut result: HashSet<i64> = HashSet::new();
        for i in 1..n - k + 2 {
            possible_products(n - i, k - 1).iter().for_each(|p| {
                // println!("n={n}, k={k}, i={i}, p={p}");
                result.insert(i * p);
            });
        }
        result
    }
}

fn minimal_product_for_k(k: i64) -> i64 {
    for n in 2.. {
        if possible_products(n, k).contains(&n) {
            return n;
        }
    }
    return -1;
}

fn solution() -> i64 {
    (2..12_001)
        .progress_count(11_999)
        .map(minimal_product_for_k)
        .sum()
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 88,
        implementations: &[("", solution)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible_products() {
        assert_eq!(possible_products(1, 1), HashSet::from([1]));
        assert_eq!(possible_products(2, 1), HashSet::from([2]));
        assert_eq!(possible_products(2, 2), HashSet::from([1]));
        assert_eq!(possible_products(3, 2), HashSet::from([2]));
        assert_eq!(possible_products(4, 2), HashSet::from([3, 4]));
    }

    #[test]
    fn test_minimal_product_for_k() {
        assert_eq!(minimal_product_for_k(2), 4);
        assert_eq!(minimal_product_for_k(3), 6);
        assert_eq!(minimal_product_for_k(4), 8);
        assert_eq!(minimal_product_for_k(5), 8);
        assert_eq!(minimal_product_for_k(6), 12);
    }
}
