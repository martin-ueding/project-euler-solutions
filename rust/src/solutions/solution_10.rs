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
        name: Some("prime list"),
        solve: solution_list,
        solution: Some(142913828922),
    }
}
inventory::submit! {
    crate::registry::SolutionEntry {
        id: 10,
        name: Some("prime sieve"),
        solve: solution_sieve,
        solution: Some(142913828922),
    }
}
