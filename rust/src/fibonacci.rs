use num_bigint::BigInt;
use num_traits::{One, Zero};

pub struct FibonacciIterator {
    a: i64,
    b: i64,
}

impl FibonacciIterator {
    pub fn new() -> Self {
        FibonacciIterator { a: 0, b: 1 }
    }
}

impl Iterator for FibonacciIterator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.b;
        let c = self.a + self.b;
        self.a = self.b;
        self.b = c;
        Some(result)
    }
}

pub struct FibonacciSuffixIterator {
    a: i64,
    b: i64,
}

impl FibonacciSuffixIterator {
    pub fn new() -> Self {
        FibonacciSuffixIterator { a: 0, b: 1 }
    }
}

impl Iterator for FibonacciSuffixIterator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.b;
        let c = (self.a + self.b) % 1_000_000_000;
        self.a = self.b;
        self.b = c;
        Some(result)
    }
}

pub struct BigFibonacciIterator {
    a: BigInt,
    b: BigInt,
}

impl BigFibonacciIterator {
    pub fn new() -> Self {
        BigFibonacciIterator {
            a: BigInt::zero(),
            b: BigInt::one(),
        }
    }
}

impl Iterator for BigFibonacciIterator {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let c = &self.a + &self.b;
        self.a = std::mem::replace(&mut self.b, c);
        Some(self.a.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_iterator_first_10() {
        assert_eq!(
            FibonacciIterator::new().take(10).collect::<Vec<i64>>(),
            vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
        );
    }

    #[test]
    fn fibonacci_suffix_iterator_first_10() {
        assert_eq!(
            FibonacciSuffixIterator::new()
                .take(10)
                .collect::<Vec<i64>>(),
            vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
        );
    }

    #[test]
    fn big_fibonacci_iterator_first_10() {
        assert_eq!(
            BigFibonacciIterator::new()
                .take(10)
                .collect::<Vec<BigInt>>(),
            vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]
                .into_iter()
                .map(BigInt::from)
                .collect::<Vec<BigInt>>()
        );
    }
}
