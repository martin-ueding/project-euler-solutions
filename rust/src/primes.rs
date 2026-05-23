use std::collections::HashMap;

pub struct PrimeGenerator {
    primes: Vec<i64>,
}

impl PrimeGenerator {
    pub fn new() -> Self {
        PrimeGenerator { primes: vec![2] }
    }

    pub fn iter(&mut self) -> PrimeIterator<'_> {
        PrimeIterator {
            primes: &mut self.primes,
            index: 0,
        }
    }
}

pub struct PrimeIterator<'a> {
    primes: &'a mut Vec<i64>,
    index: usize,
}

impl<'a> Iterator for PrimeIterator<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.primes.len() {
            let result = self.primes[self.index];
            self.index += 1;
            return Some(result);
        } else {
            let mut candidate = self.primes.last()? + 1;

            loop {
                let mut is_prime = true;
                for prime in self.primes.iter() {
                    if prime * prime > candidate {
                        break;
                    }
                    if candidate % prime == 0 {
                        is_prime = false;
                        break;
                    }
                }
                if is_prime {
                    self.primes.push(candidate);
                    self.index += 1;
                    return Some(candidate);
                }
                candidate += 1;
            }
        }
    }
}

pub fn get_prime_factors(
    mut number: i64,
    prime_generator: &mut PrimeGenerator,
) -> HashMap<i64, i64> {
    let mut factors: HashMap<i64, i64> = HashMap::new();
    for prime in prime_generator.iter() {
        while number % prime == 0 {
            if !factors.contains_key(&prime) {
                factors.insert(prime, 0);
            }
            let count = factors.get_mut(&prime).unwrap();
            *count += 1;
            number /= prime;
        }
        if number == 1 {
            break;
        }
    }
    factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_generator() {
        let mut pg = PrimeGenerator::new();
        let actual: Vec<i64> = pg.iter().take(5).collect();
        let expected = vec![2, 3, 5, 7, 11];
        assert_eq!(actual, expected);
    }

    #[test]
    fn prime_generator_reuse() {
        let mut pg = PrimeGenerator::new();
        let _: Vec<i64> = pg.iter().take(5).collect();
        let actual: Vec<i64> = pg.iter().take(5).collect();
        let expected = vec![2, 3, 5, 7, 11];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_prime_factors() {
        let mut pg = PrimeGenerator::new();
        let actual = get_prime_factors(12, &mut pg);
        let expected = HashMap::from([(2, 2), (3, 1)]);
        assert_eq!(actual, expected);
    }
}
