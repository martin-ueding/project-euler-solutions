pub fn solution() -> i64 {
    let number: i64 = 600_851_475_143;

    let mut primes: Vec<i64> = vec![2];

    let mut remainder = number;
    let mut solution: i64 = 1;
    while remainder != 1 {
        for prime in PrimeIterator::new(&mut primes) {
            if remainder % prime == 0 {
                remainder = remainder / prime;
                solution = prime;
                break;
            }
        }
    }
    solution
}

struct PrimeIterator<'a> {
    primes: &'a mut Vec<i64>,
    index: usize,
}

impl<'a> PrimeIterator<'a> {
    fn new(primes: &'a mut Vec<i64>) -> Self {
        PrimeIterator {
            primes: primes,
            index: 0,
        }
    }
}

impl<'a> Iterator for PrimeIterator<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut candidate = self.primes[self.index];

        loop {
            let mut is_prime = true;
            for prime in &mut *self.primes {
                if candidate % *prime == 0 {
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
