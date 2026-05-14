use crate::primes::PrimeGenerator;

pub fn solution() -> i64 {
    let number: i64 = 600_851_475_143;

    let mut prime_generator = PrimeGenerator::new();

    let mut remainder = number;
    let mut solution: i64 = 1;
    for prime in prime_generator.iter() {
        while remainder % prime == 0 {
            remainder = remainder / prime;
        }
        if remainder == 1 {
            solution = prime;
            break;
        }
    }
    solution
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 3,
        solve: solution,
    }
}
