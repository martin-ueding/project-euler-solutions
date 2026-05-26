use crate::primes::{PrimeList, sieve_primes};

fn solution_list() -> i64 {
    PrimeList::new()
        .iter()
        .take_while(|&prime| prime < 2_000_000)
        .sum()
}

fn solution_sieve() -> i64 {
    sieve_primes(2_000_000).iter().sum()
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 10,
        implementations: &[("prime list", solution_list), ("prime sieve", solution_sieve)],
        solution: Some(142913828922),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_list() {
        assert_eq!(solution_list(), 142_913_828_922);
    }
}
