use crate::primes::{PrimeList, sieve_primes};

fn solution_prime_list() -> i64 {
    let mut pg = PrimeList::new();
    pg.iter().take(10001).last().unwrap_or_default()
}

fn solution_sieve() -> i64 {
    let base: i64 = 10;
    for exp in 1..10 {
        let primes = sieve_primes(base.pow(exp));
        if primes.len() > 10000 {
            return primes[10000];
        }
    }
    return 0;
}

fn solution_sieve_known_end() -> i64 {
    sieve_primes(110000).get(10000).copied().unwrap_or_default()
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 7,
        implementations: &[("prime list", solution_prime_list), ("sieve", solution_sieve), ("sieve with known end", solution_sieve_known_end)],
        solution: Some(104_743),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_prime_list() {
        assert_eq!(solution_prime_list(), 104_743);
    }
}
