//! Prime related functions and generators.

use itertools::{Itertools, max};
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

pub fn get_factorizations(
    number: i64,
    max_divisor: i64,
    prime_generator: &mut PrimeList,
    cache: &mut HashMap<(i64, i64), Vec<Vec<i64>>>,
) {
    // println!("number = {number}, max_divisor = {max_divisor:?}");
    if cache.contains_key(&(number, max_divisor)) {
        return;
    }
    let mut result: Vec<Vec<i64>> = Vec::new();
    if number > 1 {
        let prime_factors = get_prime_factors(number, prime_generator);

        let product_iterator = prime_factors
            .values()
            .map(|&count| 0..count + 1)
            .multi_cartesian_product();

        for factor_counts in product_iterator {
            let divisor = prime_factors
                .keys()
                .copied()
                .zip_eq(factor_counts)
                .map(|(factor, count)| factor.pow(count as u32))
                .fold(1i64, |acc, x| acc * x);

            if divisor == 1 {
                if number > max_divisor {
                    continue;
                }
                result.push(vec![number]);
                continue;
            }

            if divisor > max_divisor {
                continue;
            }
            get_factorizations(
                number / divisor,
                min(divisor, number / divisor),
                prime_generator,
                cache,
            );

            let sub_factorizations = cache
                .get(&(number / divisor, min(divisor, number / divisor)))
                .unwrap();
            for sub_factorization in sub_factorizations.iter().cloned() {
                result.push_mut(sub_factorization).push(divisor);
            }
        }
    }
    cache.insert((number, max_divisor), result);
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
    fn test_get_factorization() {
        let mut prime_generator = PrimeList::new();
        let mut cache: HashMap<(i64, i64), Vec<Vec<i64>>> = HashMap::new();

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
        get_factorizations(24, 24, &mut prime_generator, &mut cache);
        let actual = cache.get(&(24, 24)).unwrap().clone().sort();
        assert_eq!(expected, actual);
    }
}
