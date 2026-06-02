use itertools::Itertools;
use std::collections::HashMap;

use crate::primes::{PrimeList, get_factorizations};

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
    fn test_solution_fixed_n() {
        assert_eq!(solution_fixed_n(), 7_587_457);
    }
}
