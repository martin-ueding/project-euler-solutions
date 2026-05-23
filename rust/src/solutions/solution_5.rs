use std::{cmp::max, collections::HashMap};

use crate::primes::{PrimeList, get_prime_factors};

fn solution() -> i64 {
    let mut prime_generator = PrimeList::new();
    let mut factors: HashMap<i64, i64> = HashMap::new();
    for i in 1..21 {
        let new_factors = get_prime_factors(i, &mut prime_generator);
        merge_factors(&mut factors, new_factors);
    }
    let mut result = 1;
    for (factor, count) in factors {
        result *= factor.pow(count.try_into().unwrap());
    }
    result
}

fn merge_factors(factors_1: &mut HashMap<i64, i64>, factors_2: HashMap<i64, i64>) -> () {
    for (factor, count_2) in factors_2 {
        let count_1 = factors_1.get(&factor).copied().unwrap_or(0);
        factors_1.insert(factor, max(count_1, count_2));
    }
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 5,
        solve: solution,
    }
}
