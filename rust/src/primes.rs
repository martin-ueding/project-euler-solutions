//! Prime related functions and generators.

use itertools::Itertools;
use std::{cmp::min, collections::HashMap};

/// Computes prime numbers as they are iterated.
///
/// The iterator will always start from the beginning. Already computed primes will be iterated first, afterwards new primes are computed using all previous primes.
pub struct PrimeList {
    primes: Vec<i64>,
}

impl PrimeList {
    pub fn new() -> Self {
        PrimeList { primes: vec![2] }
    }

    pub fn iter(&mut self) -> PrimeIterator<'_> {
        PrimeIterator {
            primes: &mut self.primes,
            index: 0,
        }
    }
}

/// Iterates over a PrimeList and computes primes as needed.
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

/// Factor a number into its prime factor.
///
/// The result maps prime factors to their multiples.
pub fn get_prime_factors(mut number: i64, prime_generator: &mut PrimeList) -> HashMap<i64, i64> {
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

/// Use the Sieve of Eratosthenes to compute a list of primes up to a ceiling.
pub fn sieve_primes(end: i64) -> Vec<i64> {
    let mut sieve: Vec<bool> = Vec::new();
    sieve.resize(end as usize, true);
    sieve[0] = false;
    sieve[1] = false;
    for candidate in 0..end {
        if sieve[candidate as usize] {
            for multiplier in 2..(end + candidate - 1) / candidate {
                let number = multiplier * candidate;
                sieve[number as usize] = false;
            }
        }
    }
    (0..end)
        .filter(|candidate| sieve[*candidate as usize])
        .collect()
}

/// Computes the numbers of divisors of a number.
///
/// This uses the prime factor decomposition internally.
pub fn get_num_divisors(number: i64, prime_generator: &mut PrimeList) -> i64 {
    let prime_factors = get_prime_factors(number, prime_generator);
    prime_factors
        .iter()
        .fold(1, |acc: i64, (&_prime, &count)| acc * (count + 1))
}

/// Get a sorted list of all possible divisors, including 1 and the number itself.
pub fn get_divisors(number: i64, prime_generator: &mut PrimeList) -> Vec<i64> {
    let prime_factors = get_prime_factors(number, prime_generator);

    let product_iterator = prime_factors
        .values()
        .map(|&count| 0..count + 1)
        .multi_cartesian_product();

    let mut result: Vec<i64> = Vec::new();
    for factor_counts in product_iterator {
        let divisor = prime_factors
            .keys()
            .copied()
            .zip_eq(factor_counts)
            .map(|(factor, count)| factor.pow(count as u32))
            .fold(1i64, |acc, x| acc * x);
        result.push(divisor);
    }
    result.sort();
    result
}

/// Factorizes integers while reusing sub-factorizations.
pub struct Factorizer<'a> {
    cache: HashMap<(i64, i64), Vec<Vec<i64>>>,
    prime_generator: &'a mut PrimeList,
}

impl<'a> Factorizer<'a> {
    pub fn new(prime_generator: &'a mut PrimeList) -> Self {
        Factorizer {
            cache: HashMap::new(),
            prime_generator,
        }
    }

    /// Factorizes a number without constraint.
    pub fn factorize(&mut self, number: i64) -> &Vec<Vec<i64>> {
        self.factorize_with_limit(number, number)
    }

    /// Factorizes an integer such that no factor is larger than the maximum.
    fn factorize_with_limit(&mut self, number: i64, max_divisor: i64) -> &Vec<Vec<i64>> {
        self.populate_cache(number, max_divisor);
        self.cache.get(&(number, max_divisor)).unwrap()
    }

    /// Populates the cache but doesn't return.
    ///
    /// This separation is needed to use the cache in recursive calls. Otherwise we would be using mutable and immutable references to the same cache object, which isn't possible.
    fn populate_cache(&mut self, number: i64, max_divisor: i64) {
        if self.cache.contains_key(&(number, max_divisor)) {
            return;
        }
        let mut result: Vec<Vec<i64>> = Vec::new();
        if number <= max_divisor {
            result.push(vec![number]);
        }
        for divisor in get_divisors(number, self.prime_generator)
            .into_iter()
            .skip(1)
            .take_while(|&d| d <= max_divisor)
        {
            let sub_factorizations =
                self.factorize_with_limit(number / divisor, min(divisor, number / divisor));
            for sub_factorization in sub_factorizations.iter().cloned() {
                result.push_mut(sub_factorization).push(divisor);
            }
        }
        self.cache.insert((number, max_divisor), result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_list_first_elements() {
        let mut pg = PrimeList::new();
        let actual: Vec<i64> = pg.iter().take(5).collect();
        let expected = vec![2, 3, 5, 7, 11];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_prime_list_reuse() {
        let mut pg = PrimeList::new();
        let _: Vec<i64> = pg.iter().take(5).collect();
        let actual: Vec<i64> = pg.iter().take(5).collect();
        let expected = vec![2, 3, 5, 7, 11];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_prime_factors() {
        let mut pg = PrimeList::new();
        let actual = get_prime_factors(12, &mut pg);
        let expected = HashMap::from([(2, 2), (3, 1)]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_prime_sieve() {
        let actual = sieve_primes(15);
        let expected = vec![2, 3, 5, 7, 11, 13];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_get_num_divisors() {
        let mut prime_generator = PrimeList::new();
        assert_eq!(get_num_divisors(28, &mut prime_generator), 6);
    }

    #[test]
    fn test_get_divisors() {
        let mut prime_generator = PrimeList::new();
        assert_eq!(
            get_divisors(24, &mut prime_generator),
            vec![1, 2, 3, 4, 6, 8, 12, 24]
        );
    }

    #[test]
    fn test_get_factorization() {
        let mut prime_generator = PrimeList::new();
        let mut factorizer = Factorizer::new(&mut prime_generator);

        let expected = vec![
            vec![24],
            vec![12, 2],
            vec![8, 3],
            vec![6, 4],
            vec![6, 2, 2],
            vec![4, 3, 2],
            vec![3, 2, 2, 2],
        ]
        .sort();
        let actual = factorizer.factorize(24).clone().sort();
        assert_eq!(expected, actual);
    }
}
