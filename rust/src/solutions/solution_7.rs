use crate::primes::PrimeGenerator;

fn solution() -> i64 {
    let mut pg = PrimeGenerator::new();
    pg.iter().take(10001).last().unwrap_or_default()
}


inventory::submit! {
    crate::registry::SolutionEntry {
        id: 7,
        solve: solution,
    }
}
