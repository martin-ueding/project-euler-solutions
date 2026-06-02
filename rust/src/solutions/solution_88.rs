use indicatif::ProgressIterator;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::primes::{PrimeList, get_factorizations};

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

fn solution_fixed_k() -> i64 {
    (2..12_001)
        .progress_count(11_999)
        .map(minimal_product_for_k)
        .sum()
}

fn solution_fixed_n() -> i64 {
    let mut prime_generator = PrimeList::new();

    let mut min_n_for_k: HashMap<i64, i64> = HashMap::new();

    for n in 2.. {
        let factorizations = get_factorizations(n, None, &mut prime_generator);
        for factorization in factorizations.iter() {
            if factorization.len() == 1 {
                continue;
            }
            let sum: i64 = factorization.iter().sum();
            if sum <= n {
                let diff = n - sum;
                let k: i64 = factorization.len() as i64 + diff;
                if k > 12_000 {
                    continue;
                }
                if !min_n_for_k.contains_key(&k) {
                    // println!("n = {n}, k = {k}");
                    min_n_for_k.insert(k, n);
                }
            }
        }
        if min_n_for_k.len() == 11_999 {
            break;
        }
    }

    (2..12_001)
        .map(|k| min_n_for_k.get(&k).unwrap())
        .unique()
        .sum()
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 88,
        implementations: &[("fixed n", solution_fixed_n)],
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
