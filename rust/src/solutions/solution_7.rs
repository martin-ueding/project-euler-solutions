use crate::primes::PrimeList;

fn solution_prime_list() -> i64 {
    let mut pg = PrimeList::new();
    pg.iter().take(10001).last().unwrap_or_default()
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 7,
        name: Some("prime list"),
        solve: solution_prime_list,
        solution: Some(104743),
    }
}
