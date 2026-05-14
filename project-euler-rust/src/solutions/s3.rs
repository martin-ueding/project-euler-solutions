pub fn solution() -> i64 {
    let number: i64 = 600_851_475_143;

    let mut remainder = number;
    let mut solution: i64 = 1;
    while remainder != 1 {
        for prime in PrimeIterator::new() {
            if remainder % prime == 0 {
                remainder = remainder / prime;
                solution = prime;
                break;
            }
        }
    }
    solution
}

struct PrimeIterator {
    primes: Vec<i64>,
}

impl PrimeIterator {
    pub fn new() -> Self {
        Self { primes: Vec::new() }
    }
}

impl Iterator for PrimeIterator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut candidate = match self.primes.last() {
            Some(last) => *last + 1,
            None => 2,
        };
        loop {
            let mut is_prime = true;
            for prime in &self.primes {
                if candidate % prime == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                self.primes.push(candidate);
                return Some(candidate);
            }
            candidate += 1;
        }
    }
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 3,
        solve: solution,
    }
}
