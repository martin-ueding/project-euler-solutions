use crate::primes::PrimeList;

fn solution() -> i64 {
    let mut pg = PrimeList::new();
    pg.iter().take(10001).last().unwrap_or_default()
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 7,
        solve: solution,
    }
}
